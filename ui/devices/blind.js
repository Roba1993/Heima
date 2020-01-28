import Toucher from './../toucher';

class HeimaBlind extends HTMLElement {
    connectedCallback() {
        this.device = store.get_device(this.getAttribute("id"));

        // create shadow dom
        this.attachShadow({ mode: 'open' });

        // create and add the icon and toucher
        this.toucher = new Toucher();
        this.toucher.innerHTML = `<live-icon-blind open="${this.device.status["Blind"].open}" angle="${this.device.status["Blind"].angle}"></live-icon-blind>`;
        this.shadowRoot.appendChild(this.toucher);

        // set the toucher actions
        this.toucher.onShortClick = this.toggleBlind.bind(this);
        this.toucher.onMediumClick = this.angleBlind.bind(this);
        this.toucher.onLongClick = this.angleBlindClose.bind(this);
        this.toucher.onMove = this.dimBlind.bind(this);

        this.device.addListener(this, null, this.update.bind(this));
    }

    disconnectedCallback() {
        this.device.removeListener(this);
    }

    toggleBlind() {
        // get the icon
        var icon = this.toucher.querySelector("live-icon-blind");

        // set the new state
        if (icon.getAttribute("open") === "0") {
            this.device.updateStatus("Blind", { open: 1.0 });
        }
        else {
            this.device.updateStatus("Blind", { open: 0 });
        }
    }

    dimBlind(change) {
        // get the icon
        var icon = this.toucher.querySelector("live-icon-blind");

        // scale down the input
        var change = change / 5000;

        // get the old icon value as float
        var old = parseFloat(icon.getAttribute("open"));

        // max 1 and min 0
        var change = Math.min(1, Math.max(0, old - change));

        // set the new value
        this.device.updateStatus("Blind", { open: change });
    }

    angleBlind() {
        // get the icon
        var icon = this.toucher.querySelector("live-icon-blind");

        // get the old icon value as float
        var old = parseFloat(icon.getAttribute("angle"));

        var change = Math.min(old + 0.25, 1);

        // set the new value
        this.device.updateStatus("Blind", { angle: change });
    }

    angleBlindClose() {
        this.device.updateStatus("Blind", { angle: 0 });
    }

    update() {
        let icon = this.toucher.querySelector("live-icon-blind");
        icon.setAttribute("open", this.device.status["Blind"].open);
        icon.setAttribute("angle", this.device.status["Blind"].angle);
    }
}
customElements.define("heima-blind", HeimaBlind);