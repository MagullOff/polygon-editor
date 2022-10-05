async function main() {
    const lib = await import("../pkg/index.js").catch(console.error);
    const canvas = document.getElementById("board");
    const e = canvas.getContext('2d');
    const canvasRef = lib.Canvas.new(document);

    var elemLeft = canvas.offsetLeft + canvas.clientLeft;
    var elemTop = canvas.offsetTop + canvas.clientTop;

    canvas.addEventListener('click', (event) => {
        var x = event.pageX - elemLeft,
            y = event.pageY - elemTop;
        canvasRef.onclick(x,y);
    });

    canvas.addEventListener('mousemove', (event) => {
        e.clearRect(0,0,1000,700);
        var x = event.pageX - elemLeft,
            y = event.pageY - elemTop;
        canvasRef.movemouse(x,y);
    });
}

main();
