import { Board, GameState, Cell } from "tutorial";
import { memory } from "tutorial/tutorial_bg.wasm";

const board = Board.new();

const cells = document.querySelectorAll(".cell");

const click = (row, col) => {
  board.check_box(row, col);
  checkWin(board.game_state());
  drawCells();
};
const checkWin = (gameState) => {
  switch (gameState) {
    case GameState.CircleWon:
      alert("Circle Won!");
      break;
    case GameState.CrossWon:
      alert("Cross Won!");
      break;
    case GameState.Draw:
      alert("Draw");
      break;
    case GameState.GameOn:
      break;
  }
};
const drawCells = () => {
  const cellsPtr = board.cells();
  const cellsBuffer = new Uint8Array(memory.buffer, cellsPtr, 9);
  cells.forEach((cell, idx) => {
    let symbol;
    switch (cellsBuffer[idx]) {
      case Cell.Circle:
        symbol = "O";
        break;
      case Cell.Cross:
        symbol = "X";
        break;
      case Cell.Empty:
        symbol = "";
        break;
      default:
        symbol = "";
        break;
    }
    cell.innerText = symbol;
  });
};
cells.forEach((cell) => {
  cell.onclick = () => click(parseInt(cell.id[0]), parseInt(cell.id[2]));
});
