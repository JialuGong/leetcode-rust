/**
 * @param {character[][]} grid
 * @return {number}
 */
var numIslands = function(grid) {
   let ilandNum = 0;
    let m = grid.length;
    if(m===0) return 0;
    let n = grid[0].length;
    console.log(`m is ${m}, n is ${n}`);
    for (let i = 0; i < m; i++) {
        for (let j = 0; j < n; j++) {
            if (grid[i][j] === "1") {
                ilandNum++;
                let stack = [];
                stack.push({ x: i, y: j });
                while (stack.length != 0) {
                    let currentDot = stack.pop();
                    let cX = currentDot.x;
                    let cY = currentDot.y;
                    if (grid[cX][cY] === "1") {

                        grid[cX][cY] = "0";
                        if (cX > 0 && grid[cX - 1][cY] === "1") {
                            stack.push({ x: cX - 1, y: cY });
                        }
                        if (cX + 1 < m && grid[cX + 1][cY] === "1") {
                            stack.push({ x: cX + 1, y: cY });
                        }
                        if (cY > 0 && grid[cX][cY - 1] === "1") {
                            stack.push({ x: cX, y: cY - 1 });
                        }
                        if (cY + 1 < n && grid[cX][cY + 1] === "1") {
                            stack.push({ x: cX, y: cY + 1 });
                        }
                    }
                }
            }

        }
    }

    return ilandNum;
};
