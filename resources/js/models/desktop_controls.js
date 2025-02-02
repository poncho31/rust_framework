
export class DesktopControls {

    constructor() {

        setInterval(this.updateClock, 1000);
        this.updateClock();

        document.querySelectorAll('.modal, .desktop-icon').forEach(windowEl => {
            windowEl.addEventListener('mousedown', e => {
              this.handleDragOrResize(windowEl, e);
            });
          });
          
          document.querySelectorAll('.modal').forEach(windowEl => {
            windowEl.addEventListener('mousemove', e => {
              this.updateModalCursor(windowEl, e);
            });
          });
          
          

    }


    handleDragOrResize(windowEl, e) {
        e.preventDefault();
        const rect = windowEl.getBoundingClientRect();
        const posX = e.clientX - rect.left;
        const posY = e.clientY - rect.top;
        const threshold = 10; // zone de 10px pour redimensionner
      
        // Si le clic est dans la zone de redimensionnement (bord droit ou bas)
        if (posX >= windowEl.offsetWidth - threshold || posY >= windowEl.offsetHeight - threshold) {
          e.stopPropagation(); // Empêche le drag
          windowEl.classList.add('resizing'); 
          const startX = e.clientX, startY = e.clientY;
          const startWidth = windowEl.offsetWidth, startHeight = windowEl.offsetHeight;
          
          const onMouseMoveResize = e => {
            windowEl.style.width  = (startWidth + e.clientX - startX) + 'px';
            windowEl.style.height = (startHeight + e.clientY - startY) + 'px';
          };
          
          const onMouseUpResize = () => {
            windowEl.classList.remove('resizing');
            document.removeEventListener('mousemove', onMouseMoveResize);
            document.removeEventListener('mouseup', onMouseUpResize);
          };
          
          document.addEventListener('mousemove', onMouseMoveResize);
          document.addEventListener('mouseup', onMouseUpResize);
          return;
        }
        
        // Sinon, mode déplacement (drag)
        const offsetX = e.clientX - rect.left;
        const offsetY = e.clientY - rect.top;
        let isDragging = false;
        
        const onMouseMove = e => {
          if (!isDragging) {
            if (Math.abs(e.clientX - (rect.left + offsetX)) > 5 ||
                Math.abs(e.clientY - (rect.top + offsetY)) > 5) {
              isDragging = true;
            }
          }
          if (isDragging) {
            windowEl.style.left = `${e.clientX - offsetX}px`;
            windowEl.style.top  = `${e.clientY - offsetY}px`;
          }
        };
        
        const onMouseUp = () => {
          document.removeEventListener('mousemove', onMouseMove);
          document.removeEventListener('mouseup', onMouseUp);
          if (isDragging) {
            windowEl.addEventListener('click', cancelClick, true);
            setTimeout(() => {
              windowEl.removeEventListener('click', cancelClick, true);
            }, 0);
          }
        };
        
        const cancelClick = e => {
          e.stopImmediatePropagation();
          e.preventDefault();
        };
        
        document.addEventListener('mousemove', onMouseMove);
        document.addEventListener('mouseup', onMouseUp);
      }
      

       updateModalCursor(modal, e) {
        const rect = modal.getBoundingClientRect();
        const threshold = 10; // zone de 10px en bordure
        const nearLeft   = (e.clientX - rect.left) < threshold;
        const nearRight  = (rect.right - e.clientX) < threshold;
        const nearTop    = (e.clientY - rect.top) < threshold;
        const nearBottom = (rect.bottom - e.clientY) < threshold;
        
        if (nearLeft && nearTop) {
          modal.style.cursor = 'nw-resize';
        } else if (nearRight && nearTop) {
          modal.style.cursor = 'ne-resize';
        } else if (nearLeft && nearBottom) {
          modal.style.cursor = 'sw-resize';
        } else if (nearRight && nearBottom) {
          modal.style.cursor = 'se-resize';
        } else if (nearLeft || nearRight) {
          modal.style.cursor = 'ew-resize';
        } else if (nearTop || nearBottom) {
          modal.style.cursor = 'ns-resize';
        } else {
          modal.style.cursor = 'move';
        }
      }
      
      


    
    // Open, close, minimize, fullscreen
    openWindow($this) {
        let id_modal   = $this.id + '_modal'; 
        let id_content = $this.id + '_content'; 
        // Visez l'élément "id_modal "
        const w = document.getElementById(id_modal);
        w.style.display = 'block';
        this.addTaskbarItem(id_modal );
      
        const hiddenContent = document.getElementById(id_content);
        if (hiddenContent) {
          // Insérer le contenu dans la partie "modal_content" (ou directement w.innerHTML)
          w.querySelector('.modal_content').innerHTML = hiddenContent.innerHTML;
          hiddenContent.style.display = 'none'; 
        }
    }
      


    closeWindow(id) {
        document.getElementById(id).style.display = 'none';
        this.removeTaskbarItem(id);
    }

    minimizeWindow(id) {
        document.getElementById(id).style.display = 'none';
    }

    fullscreenWindow(id) {
        const w = document.getElementById(id);
        if (w.classList.contains('fullscreen')) {
            // Si déjà en fullSize, revenir à la taille par défaut
            w.classList.remove('fullscreen');
            w.style.top = '50px';
            w.style.left = '50px';
            w.style.width = '400px';
            w.style.height = '300px';
        } else {
            // Sinon, passer en fullSize
            w.classList.add('fullscreen');
            w.style.top = '0';
            w.style.left = '0';
            w.style.width = '100%';
            w.style.height = 'calc(100% - 40px)';
        }
    }


      removeTaskbarItem(id) {
        const el = document.getElementById('taskbar-item-' + id);
        if (el) {
          el.remove();
        }
      }

      // Start menu
      toggleStartMenu() {
        const menu         = document.getElementById('startMenu');
        menu.style.display = menu.style.display === 'block' ? 'none' : 'block';
      }


    // Taskbar
    addTaskbarItem(id) {
         const window          = document.getElementById(id);
         const bar             = document.getElementById('taskbarItems');
         const taskbar_item_id = 'taskbar-item-' + id;

         if (!document.getElementById(taskbar_item_id)) {
            const taskbar_item          = document.createElement('div');

            const label = window.querySelector('.modal_header_label').textContent;

            taskbar_item.id             = taskbar_item_id;
            taskbar_item.className      = 'taskbar-item';
            taskbar_item.textContent    = label.length > 10 ? label.substring(0, 20) + "..." : label;
            taskbar_item.title          = label;
            taskbar_item.style.color    = 'white';
            taskbar_item.style.padding  = '5px 10px';
            taskbar_item.style.cursor   = 'pointer';

            taskbar_item.onclick = () => {
                document.getElementById(id).style.display = 'block';
            };

            bar.appendChild(taskbar_item);
        }
    }

    updateClock() {
        const now = new Date();
        document.getElementById('clock').textContent = now.toLocaleTimeString();
    }

}