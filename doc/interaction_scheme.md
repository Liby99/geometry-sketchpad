# Interaction Scheme

## Global interactions

- Scroll to move the viewport around

## Tool mode change

| Key | Action | Interactions |
|-----|--------|--------------|
| `S` | Change to select tool | Click to select one element, Drag a point (that can be moved) to move, Drag on empty spaces to use select rectangle to select elements that intersect with the rectangle |
| `V` | Change to viewport drag mode | Drag to move the viewport around |
| `P` | Change to draw point mode | Click on empty space to draw a free point, click on a place close to a line or intersection to draw the point on line or on the intersection |
| `L` | Change to draw line mode | Based on draw point mode, click once to set the first point of line, click the second time to set the second point, and a line will be drawn. When you want to abort the line creation after placing the first point, press `Escape` |
| `C` | Change to draw circle mode | Based on draw point mode, click once to set the center of circle, click the second time to set a point on the circle. |

## Hot Keys

| Key Stroke | Action | Descriptions |
|------------|--------|--------------|
| `Cmd - A`  | Select all elements |  |
| `Cmd - D`  | Deselect all elements |  |
| `Cmd - M`  | Create a mid-point | you need to select exactly two points in order to create this mid-point |
| `Delete` or `Backspace` | Remove all selected | |
| `Cmd - Shift - _` | Create parallel lines | you need to select exactly one line and whatever many points to draw a parallel line on every selected point |
| `Cmd - Shift - \` | Create perpendicular lines | you need to select exactly one line and whatever many points to draw a perpendicular line on every select point |
| `Cmd - H` | Hide selection | Hide the selected elements without deleting them |
| `Cmd - Shift - H` | Unhide all | Unhide all the hidden elements |
| `Cmd - Z`  | Undo | |
| `Cmd - Shift - Z` | Redo | |
| `Cmd - Q`  | Quit | |
| `Cmd - W`  | Quit | |