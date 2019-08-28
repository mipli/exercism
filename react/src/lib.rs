use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};

///
/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputCellID(usize);
/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub struct ComputeCellID(usize);

impl PartialOrd for ComputeCellID {
    fn partial_cmp(&self, other: &ComputeCellID) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ComputeCellID {
    fn cmp(&self, other: &ComputeCellID) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct ComputeCell<'cell, T: Debug + 'cell> {
    value: Option<T>,
    dependencies: Vec<CellID>,
    func: Box<Fn(&[T]) -> T>,
    callbacks: HashMap<usize, Box<FnMut(T) -> () + 'cell>>,
    callback_counter: usize,
}

impl<'cell, T: Debug + 'cell> ComputeCell<'cell, T> {
    fn new<F: 'static + Fn(&[T]) -> T>(func: F, dependencies: Vec<CellID>) -> Self {
        ComputeCell {
            value: None,
            dependencies: dependencies,
            func: Box::new(func),
            callbacks: HashMap::default(),
            callback_counter: 0,
        }
    }
}

pub struct Reactor<'reactor, T: Debug> {
    input_cells: Vec<T>,
    compute_cells: Vec<ComputeCell<'reactor, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'reactor, T: Copy + PartialEq + Debug> Reactor<'reactor, T> {
    pub fn new() -> Self {
        Reactor {
            input_cells: vec![],
            compute_cells: vec![],
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        let id = self.input_cells.len();
        self.input_cells.push(initial);
        InputCellID(id)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: 'static + Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let compute_id = self.compute_cells.len();
        let cell = ComputeCell::new(compute_func, dependencies.to_vec());
        self.compute_cells.push(cell);
        self.calculate(ComputeCellID(compute_id))?;
        Ok(ComputeCellID(compute_id))
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(InputCellID(i)) => self.input_cells.get(i).map(|t| t.clone()),
            CellID::Compute(ComputeCellID(i)) => self.compute_cells.get(i)?.value,
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        let InputCellID(i) = id;
        let cell = self.input_cells.get_mut(i).map(|cell| *cell = new_value);

        if cell.is_none() {
            return false;
        }

        let mut dependencies = self
            .compute_cells
            .iter()
            .enumerate()
            .filter_map(
                |(i, comp)| match comp.dependencies.contains(&CellID::Input(id)) {
                    true => Some(ComputeCellID(i)),
                    false => None,
                },
            )
            .collect::<BinaryHeap<_>>();

        while let Some(ComputeCellID(compute_id)) = dependencies.pop() {
            let cell_id = ComputeCellID(compute_id);
            // can ignore this error since all cell ids are validated upon cell creation,
            // therefore calculation cannot fail at this time
            let _ = self.calculate(cell_id);
            self.compute_cells.iter().enumerate().for_each(|(i, comp)| {
                if comp.dependencies.contains(&CellID::Compute(cell_id)) {
                    dependencies.push(ComputeCellID(i));
                }
            });
        }

        true
    }

    fn calculate(&mut self, id: ComputeCellID) -> Result<T, CellID> {
        let ComputeCellID(compute_id) = id;
        let cell = self
            .compute_cells
            .get(compute_id)
            .ok_or_else(|| CellID::Compute(ComputeCellID(compute_id)))?;

        let args = cell.dependencies.iter().try_fold(vec![], |mut args, dep| {
            let val = match dep {
                CellID::Input(InputCellID(id)) => {
                    let cell = self
                        .input_cells
                        .get(*id)
                        .ok_or_else(|| CellID::Input(InputCellID(*id)))?;
                    *cell
                }
                CellID::Compute(ComputeCellID(id)) => {
                    let cell = self
                        .compute_cells
                        .get(*id)
                        .ok_or_else(|| CellID::Compute(ComputeCellID(*id)))?;
                    cell.value
                        .ok_or_else(|| CellID::Compute(ComputeCellID(*id)))?
                }
            };
            args.push(val);
            Ok(args)
        })?;
        let result = (*cell.func)(args.as_slice());
        match self.compute_cells.get_mut(compute_id) {
            Some(cell) => {
                if cell.value.replace(result) != Some(result) {
                    for cb in cell.callbacks.values_mut() {
                        cb(result);
                    }
                }
                Ok(result)
            }
            None => Err(CellID::Compute(ComputeCellID(compute_id))),
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) -> () + 'reactor>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        let ComputeCellID(compute_id) = id;
        let compute_cell = self.compute_cells.get_mut(compute_id)?;
        let callback_id = compute_cell.callback_counter;
        compute_cell
            .callbacks
            .insert(callback_id, Box::new(callback));
        compute_cell.callback_counter += 1;
        Some(CallbackID(callback_id))
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        id: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        let ComputeCellID(compute_id) = id;
        let CallbackID(callback_id) = callback;
        let compute_cell = self
            .compute_cells
            .get_mut(compute_id)
            .ok_or_else(|| RemoveCallbackError::NonexistentCell)?;
        compute_cell
            .callbacks
            .remove(&callback_id)
            .ok_or_else(|| RemoveCallbackError::NonexistentCallback)?;
        Ok(())
    }
}
