import Toucher from './../toucher';

class HeimaDimLight extends HTMLElement {
    connectedCallback() {
        this.device = store.get_device(this.getAttribute("id"));

        // create shadow dom
        this.attachShadow({ mode: 'open' });

        // create and add the icon and toucher
        this.toucher = new Toucher();
        this.toucher.innerHTML = `<live-icon-light power="${this.device.status["DimLight"].power}"></live-icon-light>`;
        this.shadowRoot.appendChild(this.toucher);

        // set the toucher actions
        this.toucher.onShortClick = this.toggleLight.bind(this);
        this.toucher.onMediumClick = this.toggleLight.bind(this);
        this.toucher.onLongClick = this.toggleLight.bind(this);
        this.toucher.onMove = this.dimLight.bind(this);

        this.device.addListener(this, null, this.update.bind(this));
    }

    disconnectedCallback() {
        this.device.removeListener(this);
    }

    toggleLight() {
        // get the icon
        var icon = this.toucher.querySelector("live-icon-light");

        // set the new state
        if (icon.getAttribute("power") === "0") {
            this.device.updateStatus("DimLight", { power: 1.0 });
        }
        else {
            this.device.updateStatus("DimLight", { power: 0 });
        }
    }

    dimLight(change) {
        // get the icon
        var icon = this.toucher.querySelector("live-icon-light");

        // scale down the input
        var change = change / 5000;

        // get the old icon value as float
        var old = parseFloat(icon.getAttribute("power"));

        // max 1 and min 0
        var change = Math.min(1, Math.max(0, old + change));

        // set the new value
        this.device.updateStatus("DimLight", { power: change });
    }

    update() {
        this.toucher.querySelector("live-icon-light").setAttribute("power", this.device.status["DimLight"].power);
    }
}
customElements.define("heima-dim-light", HeimaDimLight);