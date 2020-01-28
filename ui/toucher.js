export default class Toucher extends HTMLElement {
    connectedCallback() {
        // check for only call once
        if (this.connected) {
            return;
        }
        this.connected = true;

        // create the touch-surface area on top, 
        // to work even when the inner content changes
        var main = document.createElement("div");
        main.id = "touch-surface";
        main.style.width = "100%";
        main.style.height = "100%";
        main.style.zIndex = "100";
        main.style.position = "absolute";
        main.style.top = "0px";
        this.appendChild(main);

        // bind this class to the event handler functions
        this.down = this.down.bind(this);
        this.move = this.move.bind(this);
        this.up = this.up.bind(this);
        this.leave = this.leave.bind(this);

        // define the mouse events
        this.onmousedown = this.down;
        this.onmousemove = this.move;
        this.onmouseup = this.up;
        this.onmouseleave = this.leave;

        // define the touch events
        this.addEventListener('touchstart', this.down);
        this.addEventListener('touchmove', this.move);
        this.addEventListener('touchend', this.up);
    }

    down(e) {
        if (this.press) {
            return;
        }

        this.time = Date.now();
        this.press = true;
        this.startX = e.type === 'mousedown' ? e.clientX : e.touches[0].clientX;
        this.startY = e.type === 'mousedown' ? e.clientY : e.touches[0].clientY;
        this.movedY = 0;
        this.movedX = 0;

        e.preventDefault();
    }

    move(e) {
        if (!this.press) {
            return;
        }

        if (e.type === 'mousemove') {
            this.movedX = this.startX - e.clientX;
            this.movedY = this.startY - e.clientY;
        }
        else if (e.type === 'touchmove') {
            this.movedX = this.startX - e.touches[0].clientX;
            this.movedY = this.startY - e.touches[0].clientY;
        }

        if (Math.abs(this.movedY) > Math.abs(this.movedX)) {
            this.onMove(this.movedY);
        }

        e.preventDefault();
    }

    up(e) {
        if (!this.press) {
            return;
        }

        var time = Date.now() - this.time;
        this.press = false;

        if (Math.abs(this.movedY) > Math.abs(this.movedX) && Math.abs(this.movedY) > 10) {
            this.onMoveEnd(this.movedY);
        }
        else if (time < 300 && Math.abs(this.movedX) < 25) {
            this.onShortClick();
        }
        else if (time < 1000 && Math.abs(this.movedX) < 25) {
            this.onMediumClick();
        }
        else if (time > 1000 && Math.abs(this.movedX) < 25) {
            this.onLongClick();
        }

        e.preventDefault();
    }

    leave() {
        this.press = false;

        if (Math.abs(this.movedY) > Math.abs(this.movedX) && Math.abs(this.movedY) > 10) {
            this.onMoveEnd(this.movedY);
        }
    }

    // function to be placed by user to get informed about changes
    onShortClick() { }
    onMediumClick() { }
    onLongClick() { }
    onMove() { }
    onMoveEnd() { }
}
customElements.define("heima-toucher", Toucher);