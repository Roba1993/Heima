class Slider extends HTMLElement {
    constructor() {
        super();

        this._width = '250px';
        this._height = "250px";
    }

    connectedCallback() {
        if (!this.shadowRoot) {
            this.attachShadow({ mode: 'open' });
            this.render();
        }

        var wrapper = this.shadowRoot.querySelector('#slider');
        var items = this.shadowRoot.querySelector('#items');
        var dots = this.shadowRoot.querySelector('#dots');
        var dotNumber = items.children.length;

        var posX1 = 0,
            posX2 = 0,
            posInitial,
            posFinal,
            threshold = 100,
            slides = items.getElementsByClassName('slide'),
            slidesLength = slides.length,
            slideSize = items.getElementsByClassName('slide')[0].offsetWidth,
            firstSlide = slides[0],
            lastSlide = slides[slidesLength - 1],
            cloneFirst = firstSlide.cloneNode(true),
            cloneLast = lastSlide.cloneNode(true),
            index = 0,
            allowShift = true;

        // Clone first and last slide
        items.appendChild(cloneFirst);
        items.insertBefore(cloneLast, firstSlide);
        wrapper.classList.add('loaded');

        // create the dots
        for (let i = 0; i < dotNumber; i++) {
            var el = document.createElement("li");
            if (i === 0) {
                el.classList.add('active');
            }

            el.onclick = () => {
                setSlide(i);
            }

            dots.appendChild(el);
        }

        // Mouse and Touch events
        items.onmousedown = dragStart;

        // Touch events
        items.addEventListener('touchstart', dragStart);
        items.addEventListener('touchend', dragEnd);
        items.addEventListener('touchmove', dragAction);

        // Transition events
        items.addEventListener('transitionend', checkIndex);

        function dragStart(e) {
            e = e || window.event;
            e.preventDefault();
            posInitial = items.offsetLeft;

            if (e.type == 'touchstart') {
                posX1 = e.touches[0].clientX;
            } else {
                posX1 = e.clientX;
                document.onmouseup = dragEnd;
                document.onmousemove = dragAction;
            }
        }

        function dragAction(e) {
            e = e || window.event;

            if (e.type == 'touchmove') {
                posX2 = posX1 - e.touches[0].clientX;
                posX1 = e.touches[0].clientX;
            } else {
                posX2 = posX1 - e.clientX;
                posX1 = e.clientX;
            }

            items.style.left = (items.offsetLeft - posX2) + "px";
        }

        function dragEnd(e) {
            posFinal = items.offsetLeft;
            if (posFinal - posInitial < -threshold) {
                shiftSlide(1, 'drag');
            } else if (posFinal - posInitial > threshold) {
                shiftSlide(-1, 'drag');
            } else {
                items.style.left = (posInitial) + "px";
            }

            document.onmouseup = null;
            document.onmousemove = null;
        }

        function shiftSlide(dir, action) {
            items.classList.add('shifting');

            if (allowShift) {
                if (!action) { posInitial = items.offsetLeft; }

                if (dir == 1) {
                    items.style.left = (posInitial - slideSize) + "px";
                    index++;
                } else if (dir == -1) {
                    items.style.left = (posInitial + slideSize) + "px";
                    index--;
                }
            };

            allowShift = false;
        }

        function setSlide(dir) {
            if (dir == index) {
                return;
            }

            items.classList.add('shifting');

            if (allowShift) {
                index = dir;
                items.style.left = (-slideSize * (index + 1)) + "px";
            };

            allowShift = false;
        }

        function checkIndex() {
            items.classList.remove('shifting');

            if (index == -1) {
                items.style.left = -(slidesLength * slideSize) + "px";
                index = slidesLength - 1;
            }

            if (index == slidesLength) {
                items.style.left = -(1 * slideSize) + "px";
                index = 0;
            }

            // update the dots
            for (let i = 0; i < dots.children.length; i++) {
                dots.children[i].classList.remove('active');
            }
            dots.children[index].classList.add('active');

            allowShift = true;
        }



    }

    render() {
        if (this.shadowRoot === null) {
            return;
        }

        this.shadowRoot.innerHTML = `
            <div id="slider" class="slider">
                <div class="wrapper">
                    <div id="items" class="items">
                        ${this.innerHTML}
                    </div>
                </div>
                <div id="dots">
                </div>
            </div>

            <style>
                .slider {
                    width: ${this._width};
                    height: ${this._height};
                }

                .wrapper {
                    overflow: hidden;
                    position: relative;
                    z-index: 1;
                }

                #items {
                    width: 10000px;
                    position: relative;
                    top: 0;
                    left: -${this._width};
                    display: block;
                }

                #items.shifting {
                    transition: left .2s ease-out;
                }

                .slide {
                    width: ${this._width};
                    height: ${this._height};
                    cursor: pointer;
                    float: left;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    transition: all 1s;
                    position: relative;
                }

                #dots {
                    position: relative;
                    width: 100px;
                    left: 50%;
                    margin-left: -50px;
                    top: 0px;
                    text-align: center;
                    z-index: 2;
                }

                #dots > li {
                    display: inline-block;
                    width: 12px;
                    height: 12px;
                    background-color: grey;
                    border-radius: 50%;
                    margin: 0 4px;
                    opacity: 0.5;
                    cursor: pointer;
                    transition: opacity 0.3s;
                }

                #dots > li:hover, #dots > li.active {
                    opacity: 1;
                }
            </style>
        `;
    }

    static get observedAttributes() {
        return ['width', 'height'];
    }


    attributeChangedCallback(name, oldVal, newVal) {
        this["_" + name] = newVal;
        this.render();
    }

    safeSetAttribute(name, value) {
        if (this.getAttribute(name) !== value) this.setAttribute(name, value);
    }

    set width(val) {
        this.safeSetAttribute("width", val);
    }

    set height(val) {
        this.safeSetAttribute("height", val);
    }
}
customElements.define("heima-slider", Slider);