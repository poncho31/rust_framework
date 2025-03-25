<!DOCTYPE html>
<html lang="fr">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>The Donuts Game</title>
        <style>
            html, body {
                margin: 0;
                padding: 0;
                width: 100vw;
                height: 100vh;
            }
            .container {
                display: flex;
                flex-wrap: wrap;
                width: 100vw;
                height: 100vh;
            }
            .square {
                flex: 0 0 20vw;
                height: 20vh;
                border: 1px solid rgba(0,0,0,0.5);
                box-sizing: border-box;
                display: flex;
                justify-content: center;
                align-items: center;
                aspect-ratio: 1/1;
                overflow: hidden;
            }

            /* DONUT */
            #donut_text_click_me{
                position: absolute;
                font-weight: bold;
                text-indent: 10px;
                top: 20;
                left: 0;
                transform: rotate(-30deg);
                transition: all 0.3s ease;
                text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.2);
                color: #FF1493;
                font-weight: bold;
            }

            #donut_button {
                cursor: pointer;
                transition: all 0.3s ease;
                filter: drop-shadow(0 4px 4px rgba(0, 0, 0, 0.2));
                max-width: 64%;
                min-width: 50%;
                height: auto;
                display: block;
                z-index: 1;
            }

            #donut_button:hover {
                transform: translateY(-2px) scale(1.05);
                filter: drop-shadow(0 6px 6px rgba(0, 0, 0, 0.3));
                width: min(95%, 95%); 
            }

            #donut_button:active {
                transform: translateY(1px) scale(0.95);
                filter: drop-shadow(0 2px 2px rgba(0, 0, 0, 0.2));
                width: min(90%, 90%); 
            }

            #donut_count{
                position: absolute;
                color: #FF1493;
                font-weight: bold;
            }

            .donut_count_level_1{
                background-color: black;
                padding: 5px;
                border-radius: 100%;
                border-color: #FF1493;
                border-width: 5px;;
            }
            .donut_square_level_1{
                background-color: black;
            }


            /* INCUBATOR */
            #incubator  {
                border: 5px solid #FF1493;
                box-sizing: border-box;
                content: "Incubator";
                font-weight: bold;
            }
            
            .incubator_level_1{
                border:#FF1493 5px solid;
                border-width: 5px;
                content: "Incubator";
                font-weight: bold;
            }


            /* LOADING SCREEN */
            #loading-screen {
                position: fixed;
                top: 0;
                left: 0;
                width: 100vw;
                height: 100vh;
                background-color: #000000;
                display: flex;
                justify-content: center;
                align-items: center;
                z-index: 9999;
            }

            .loader {
                width: 50px;
                height: 50px;
                border: 5px solid #FF1493;
                border-bottom-color: transparent;
                border-radius: 50%;
                display: inline-block;
                box-sizing: border-box;
                animation: rotation 1s linear infinite;
            }

            @keyframes rotation {
                0% {
                    transform: rotate(0deg);
                }
                100% {
                    transform: rotate(360deg);
                }
            }

            .hide-loading {
                opacity: 0;
                visibility: hidden;
                transition: all 0.5s ease-out;
            }
        </style>
    </head>
    <body>
        <div class="container">

             <!-- DONUT -->
            <div class="square" id="donut" >
                <span id="donut_text_click_me">Click me !</span>
                <img id="donut_button" src="./images/Donut.svg" alt="">

                <div id="donut_count">0</div>
            </div>

            <!-- INCUBATOR -->
            <div class="square" id="incubator">

                <!-- Employees -->
                <div class="incubator_employees">

                    <!-- manager -->
                    <table class="incubator_manager">
                        <tr><td colspan="2">Manager IT</td></tr>
                        <tr>
                            <td>
                                Count :
                            </td>
                            <td>
                                <button class="count_incubator_staff"> 0 </button>
                            </td>
                        </tr>
                    </table>

                    <hr>

                    <!-- staff -->
                    <table class="incubator_staff">
                        <tr><td colspan="2">Staff IT</td></tr>
                        <tr>
                            <td>Count :</td>
                            <td>
                                <button class="count_incubator_staff"> 0 </button>
                            </td>
                        </tr>
                        <tr>
                            <td >Actions :</td>
                            <td >
                                <button class="add_incubator_staff"  > Add Staff  </button>
                            </td>
                            <td>
                                <button class="add_incubator_staff"  > Feed Staff </button>
                            </td>
                        </tr>
                    </table>

                </div>

            </div>

            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>
            <div class="square"></div>

        </div>


        <!-- Ajout de l'écran de chargement -->
        <div id="loading-screen">
            <span class="loader"></span>
        </div>


        <script type="module">
            import { Game } from './js/game.js';

            function sleep(ms) {
                return new Promise(resolve => setTimeout(resolve, ms));
            }

            // Attendre le chargement complet et simuler un délai de 1.5 secondes
            Promise.all([
                new Promise(resolve => window.addEventListener('DOMContentLoaded', resolve)),
                new Promise(resolve => window.addEventListener('load', resolve)),
                sleep(1500)
            ]).then(() => {
                // Initialiser le jeu
                const game = new Game();
                
                // Cacher l'écran de chargement avec animation
                const loadingScreen = document.getElementById('loading-screen');
                loadingScreen.classList.add('hide-loading');
                
                // Supprimer l'élément après l'animation
                setTimeout(() => {
                loadingScreen.remove();
                }, 500);
            });
        </script>

    </body>
</html>


