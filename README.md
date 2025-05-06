# Kaji - A Logical Puzzle Solver

Kaji is a solver for orthogonal grid, cell-based puzzles
such as Sudoku.

Kaji aims to always make logical deductions rather than
brute-forcing solutions.

Kaji is maintained as part of a live-coding exercise
which can be found at: [Youtube][ytpl]

[ytpl]: https://youtube.com/playlist?list=PL_xuff3BkASM9CcXm6Qv8jJJKbB8xl9mT&si=9J-DrF9lxofs_nQt

## Ideas for the future

- Make it so that the `Puzzle::solve()` function _returns_ the logical steps
  rather than printing them out.
- Use the puzzle loader to make some specific tests eg. one where a row contains
  12345678, and check that naked-single sets the 9
- Alter the `PuzzleBuilder` so that we have multiple stages of building
  a puzzle, for example, starting with adding the cells, then the regions, etc.
  to allow for certain cached information (eg. is a region contiguous)
- Alter `SudokuGrid` to take the digits (optional) and take coordinate in
  logical grid space (optional) to root the grid.
  This is ultimately to allow for multi-grid puzzles
- Give `Cell`s the ability to have a display name (currently this is rXcY)
