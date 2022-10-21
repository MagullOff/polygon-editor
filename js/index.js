const leftButton = 0;

const canvasContainer = document.querySelector('.canvasContainer');
const canvas = document.querySelector('canvas');
const boundingRect = canvasContainer.getBoundingClientRect();
canvas.height = boundingRect.height
canvas.width = boundingRect.width

const cords = document.querySelector(".cords");

const radioCreate = document.querySelector("#Edit");
const radioEdit = document.querySelector("#Highlight");
const radioRules = document.querySelector("#Split");

const container = document.querySelector(".container");
const modalContainer = document.querySelector(".modalContainer");

const helpExitButton = document.querySelector(".closeModal");
const sceneButton = document.querySelector("#Scene");
const helpButton = document.querySelector("#Help");

const bresenhamButton = document.querySelector("#Algorithm");

const lengthSelector = document.querySelector('#LengthSelector');
const isLengthConst = document.querySelector('#IsConst');

helpExitButton.onclick = () => {
    container.classList.remove("modalShown");
    modalContainer.classList.remove("modalShown");
}

helpButton.onclick = () => {
    container.classList.add("modalShown");
    modalContainer.classList.add("modalShown");
}

const setPressedButton = (radioObject) => {
    radioCreate.classList.remove("activeMode");
    radioEdit.classList.remove("activeMode");
    radioRules.classList.remove("activeMode");

    radioObject.classList.add("activeMode");
}

async function setHandlers() {
    const lib = await import("../pkg/index.js").catch(console.error);
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
        console.log('right');
    };

    canvas.onmousedown = (event) => {
        const x = event.pageX - elemLeft,
            y = event.pageY - elemTop;
        console.log('mouse down', x, y);
        if(event.button == 0)
            canvasRef.on_down_click(x,y);
    }

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

    radioCreate.onclick = () => {
        console.log('radion click');
        setPressedButton(radioCreate);
        canvasRef.set_create_state();
    };

    radioEdit.onclick = () => {
        console.log('highlight click');
        setPressedButton(radioEdit);
        canvasRef.set_edit_state();
    };

    radioRules.onclick = () => {
        console.log('rules click');
        setPressedButton(radioRules);
        canvasRef.set_rules_state();
    };

    sceneButton.onclick = () => {
        console.log('predefined scene click');
        canvasRef.set_predefined_scene();
    }

    bresenhamButton.onclick = () => {
        console.log('bresenham click');
        canvasRef.draw_bresenham();
    }

    lengthSelector.onchange = (event) => {
        console.log('set new length', event.target.value);
        canvasRef.set_line_length();
    }

    isLengthConst.onchange = (event) => {
        console.log('const change', event.target.checked);
        canvasRef.set_const_state();
    }
}

setHandlers();
