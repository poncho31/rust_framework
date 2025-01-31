
export class DesktopControls {

    constructor() {

        document.querySelectorAll('.desktop-icons .icon').forEach(icon => {
            let isDragging = false, offsetX, offsetY;
            icon.addEventListener('mousedown', e => {
              isDragging = true;
              offsetX = e.offsetX;
              offsetY = e.offsetY;
            });
            document.addEventListener('mousemove', e => {
              if (isDragging) {
                icon.style.left = (e.pageX - offsetX) + 'px';
                icon.style.top = (e.pageY - offsetY) + 'px';
              }
            });
            document.addEventListener('mouseup', () => {
              isDragging = false;
            });
          });
          

        function updateClock() {
            const now = new Date();
            document.getElementById('clock').textContent = now.toLocaleTimeString();
        }

        setInterval(updateClock, 1000);
        updateClock();
      // Drag and drop
      document.querySelectorAll('.window').forEach(windowEl => {
        let isDragging = false;
        let offsetX, offsetY;
        const header = windowEl.querySelector('.window-header');

        header.addEventListener('mousedown', e => {
          isDragging = true;
          offsetX = e.clientX - windowEl.offsetLeft;
          offsetY = e.clientY - windowEl.offsetTop;
        });
        document.addEventListener('mousemove', e => {
          if (isDragging) {
            windowEl.style.left = `${e.clientX - offsetX}px`;
            windowEl.style.top = `${e.clientY - offsetY}px`;
          }
        });
        document.addEventListener('mouseup', () => {
          isDragging = false;
        });
      });



    }


    
      // Open, close, minimize, fullscreen
      openWindow($this) {
        let id = $this.id; 
        // Visez l'élément "id_window"
        const w = document.getElementById(id + '_window');
        w.style.display = 'block';
        addTaskbarItem(id + '_window');
      
        const hiddenContent = document.getElementById(id + '_content');
        if (hiddenContent) {
          // Insérer le contenu dans la partie "window-content" (ou directement w.innerHTML)
          w.querySelector('.window-content').innerHTML = hiddenContent.innerHTML;
          hiddenContent.style.display = 'none'; 
        }
      }
      


      closeWindow(id) {
        document.getElementById(id).style.display = 'none';
        removeTaskbarItem(id);
      }
      minimizeWindow(id) {
        document.getElementById(id).style.display = 'none';
      }
      fullscreenWindow(id) {
        const w = document.getElementById(id);
        w.style.top = '0';
        w.style.left = '0';
        w.style.width = '100%';
        w.style.height = 'calc(100% - 40px)';
      }

      removeTaskbarItem(id) {
        const el = document.getElementById('taskbar-item-' + id);
        if (el) {
          el.remove();
        }
      }

      // Start menu
      toggleStartMenu() {
        const menu = document.getElementById('startMenu');
        menu.style.display = menu.style.display === 'block' ? 'none' : 'block';
      }


            // Taskbar
    addTaskbarItem(id) {
         const bar = document.getElementById('taskbarItems');
         if (!document.getElementById('taskbar-item-' + id)) {
           const item = document.createElement('div');
           item.id = 'taskbar-item-' + id;
           item.className = 'taskbar-item';
           item.textContent = id;
           item.style.color = 'white';
           item.style.padding = '5px 10px';
           item.style.cursor = 'pointer';
           item.onclick = () => {
             document.getElementById(id).style.display = 'block';
           };
           bar.appendChild(item);
         }
       }

}