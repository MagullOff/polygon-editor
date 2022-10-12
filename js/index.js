const leftButton = 0;

const canvasContainer = document.querySelector('.canvasContainer');
const canvas = document.querySelector('canvas');
const boundingRect = canvasContainer.getBoundingClientRect();
canvas.height = boundingRect.height
canvas.width = boundingRect.width

const cords = document.querySelector(".cords");

const radioEdit = document.getElementById("Edit");
const radioHl = document.getElementById("Highlight");
const radioSplit = document.getElementById("Split");

const container = document.querySelector(".container");
const modalContainer = document.querySelector(".modalContainer");

const helpExitButton = document.querySelector(".closeModal");
const helpButton = document.querySelector("#Help");

helpExitButton.onclick = () => {
    container.classList.remove("modalShown");
    modalContainer.classList.remove("modalShown");
}

helpButton.onclick = () => {
    container.classList.add("modalShown");
    modalContainer.classList.add("modalShown");
}

const setPressedButton = (radioObject) => {
    radioEdit.classList.remove("activeMode");
    radioHl.classList.remove("activeMode");
    radioSplit.classList.remove("activeMode");

    radioObject.classList.add("activeMode");
}

async function main() {
    const lib = await import("../pkg/index.js").catch(console.error);
    const canvas = document.getElementById("board");

    const sceneButton = document.getElementById("Scene");

    const canvasRef = lib.Canvas.new(document);

    const elemLeft = canvas.offsetLeft + canvas.clientLeft;
    const elemTop = canvas.offsetTop + canvas.clientTop;

    window.onresize = () => {
        const currentBoundingRect = canvasContainer.getBoundingClientRect();
        canvas.height = currentBoundingRect.height
        canvas.width = currentBoundingRect.width
        canvasRef.draw();
    };
    canvas.oncontextmenu = (event) => {
        const x = event.pageX - elemLeft,
            y = event.pageY - elemTop;
        event.preventDefault();
        console.log('right');
        canvasRef.on_right_click(x, y);
    };

    // mouse down 
    canvas.onmousedown = (event) => {
        const x = event.pageX - elemLeft,
            y = event.pageY - elemTop;
        console.log('mouse down', x, y);
        if(event.button == 0)
            canvasRef.on_down_click(x,y);
    }

    // mouse up
    canvas.onclick = (event) => {
        const x = event.pageX - elemLeft,
            y = event.pageY - elemTop;
        console.log('mouse up',x,y);
        canvasRef.on_left_click(x, y);
    };

    canvas.onmousemove = (event) => {
        const x = event.pageX - elemLeft,
            y = event.pageY - elemTop;

        cords.innerHTML = `x: ${x.toFixed(0)} y: ${y.toFixed(0)}`;
        canvasRef.on_move_mouse(x, y);
    };

    radioEdit.onclick = () => {
        console.log('radion click');
        setPressedButton(radioEdit);
        canvasRef.set_edit_state();
    };

    radioHl.onclick = () => {
        console.log('highlight click');
        setPressedButton(radioHl);
        canvasRef.set_highlight_state();
    };

    radioSplit.onclick = () => {
        console.log('highlight click');
        setPressedButton(radioSplit);
        canvasRef.set_split_state();
    };

    sceneButton.onclick = () => {
        console.log('predefined scene click');
        canvasRef.set_predefined_scene();
    }
}

main();
