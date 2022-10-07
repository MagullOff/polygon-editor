async function main() {
    const lib = await import("../pkg/index.js").catch(console.error);
    const canvas = document.getElementById("board");
    const radioEdit = document.getElementById("Edit");
    const radioHl = document.getElementById("Highlight");
    const canvasRef = lib.Canvas.new(document);

    var elemLeft = canvas.offsetLeft + canvas.clientLeft;
    var elemTop = canvas.offsetTop + canvas.clientTop;

    canvas.oncontextmenu = (event) => {
        var x = event.pageX - elemLeft,
            y = event.pageY - elemTop;
        event.preventDefault();
        console.log('right');
        canvasRef.on_right_click(x, y);
    };

    canvas.onclick = (event) => {
        var x = event.pageX - elemLeft,
            y = event.pageY - elemTop;
        console.log('left',x,y);
        canvasRef.on_left_click(x,y);
    };

    canvas.onmousemove = (event) => {
        var x = event.pageX - elemLeft,
            y = event.pageY - elemTop;
        console.log(x,y);
        canvasRef.on_move_mouse(x,y);
    };

    radioEdit.onclick = () => {
        console.log('radion click');
        canvasRef.set_edit_state();
    };

    radioHl.onclick = () => {
        console.log('hl click');
        canvasRef.set_highlight_state();
    };
}

main();
