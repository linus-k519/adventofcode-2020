import isatty = Deno.isatty;

type Point = {
    x: number,
    y: number,
}

type Line = {
    from: Point,
    to: Point,
}


/**
 * Parses a line from string.
 */
function parseLine(lineStr: string): Line {
    const lineRegex = RegExp("(\\d+),(\\d+) -> (\\d+),(\\d+)")
    const regexResult = lineRegex.exec(lineStr) || []
    const [, x1, y1, x2, y2]: Array<string> = regexResult
    return {
        from: {
            x: Number.parseInt(x1),
            y: Number.parseInt(y1),
        },
        to: {
            x: Number.parseInt(x2),
            y: Number.parseInt(y2),
        }
    }
}

function getPointsOfLine(line: Line): Array<Point> | null {
    const vector = {
        deltaX: line.to.x - line.from.x,
        deltaY: line.to.y - line.from.y
    }

    const isHorizontal = line.from.y == line.to.y
    const isVertical = line.from.x == line.to.x
    if (!(isHorizontal || isVertical)) {
        return null;
    }

    const points: Array<Point> = []
    for (let i = 0; i <= Math.max(Math.abs(vector.deltaX), Math.abs(vector.deltaY)); i++) {
        if (isHorizontal) {
            points.push({
                x: line.from.x + Math.sign(vector.deltaX) * i,
                y: line.from.y, // Ok because line.from.y == line.to.y is always true
            })
        } else { // isVertical
            points.push({
                x: line.from.x, // Ok because line.from.x == line.to.x is always true
                y: line.from.y + Math.sign(vector.deltaY) * i,
            })
        }
    }
    return points
}


const input = await Deno.readTextFile("input.txt")
const inputLines: Array<string> = input.split("\n")

const lines: Array<Line> = inputLines.map(parseLine)

// Map<Point, number> does not work, because map checks for referential equality (same memory address), not
// structural equality (same content) for keys that are objects
const diagram = new Map<String, number>()

for (const line of lines) {
    let pointsOfLine: Array<Point> | null = getPointsOfLine(line)
    if (pointsOfLine == null) {
        // Illegal line, i.e. not horizontal, vertical or diagonal
        continue
    }
    for (const point of pointsOfLine) {
        // Store JSON representation in map, because map checks for referential equality (same memory address), not
        // structural equality (same content) for keys that are objects
        const pointJSON = JSON.stringify(point)
        // Increment counter of lines that hit this point. If not already in the map, set it to 0 and increment it
        diagram.set(pointJSON, (diagram.get(pointJSON) ?? 0) + 1);
    }
}

const pointsWithManyLines = [...diagram].filter(([point, linesOnThisPoint]) => linesOnThisPoint >= 2)


console.log(pointsWithManyLines.length)