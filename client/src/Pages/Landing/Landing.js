import React from 'react';

import logo from './../../img/logo-black.svg';
import welcome_image from './../../img/welcome-image.svg';
import arrow_icon from './../../img/arrow-icon.svg';

// CSS
import './../../App.css'
import './landing-styles.css'

export function Landing() {
    return (
        <div id="container">
            <nav>
                <div id="nav-container">
                    <div id="left-buttons">
                        <a id="home-button" href="./">
                            <img id="logo" src={logo} alt="Logo"></img>
                        </a>
                    </div>
                    <div id="right-buttons">
                        <div id="hamburger-menu" onClick={hamburgerClickEvent}>
                            <div class="bar"></div>
                            <div class="bar"></div>
                            <div class="bar"></div>
                        </div>
                        <a id="signup" class="button" href="./signup">Signup</a>
                        <a id="login" class="button" href="./login">Login</a>
                    </div>
                </div>
            </nav>
            <main>
                <div id="welcome-container">
                    <div id="welcome-message">
                        <h1>Project managment, without the hassle</h1>
                        <p>
                            Easy Tracker is a fast, effective, and  reliable
                            way to track bugs, and improve productivity.
                            Enjoy faster development cycles, like never before.
                        </p>
                    </div>
                    <div id="welcome-image-wrapper">
                        <img id="welcome-image" src={welcome_image} alt="Computer"></img>
                    </div>
                </div>
                <div id="feature-container">
                    <div class="feature">
                        <div class="feature-text">

                        </div>
                        <div class="feature-image">

                        </div>
                    </div>
                    <div class="feature">
                        <div class="feature-image">

                        </div>
                        <div class="feature-text">

                        </div>
                    </div>
                    <div class="feature">
                        <div class="feature-text">

                        </div>
                        <div class="feature-image">

                        </div>
                    </div>
                </div>
                <div id="signup-container">
                    <h2>Start now!</h2>
                    <img id="signup-icon" src={arrow_icon} alt="Signup Button"></img>
                </div>
            </main>
            <footer>
                <p>Easy Tracker &#169; 2021</p>
            </footer>
        </div>
    );
}

function hamburgerClickEvent(e) {

    let hamburger = document.getElementById("hamburger-menu");

    if (hamburger.classList.contains("show")) {

        hamburger.classList.remove("show");

    } else {

        hamburger.classList.add("show");

    }

    console.log(hamburger)

}