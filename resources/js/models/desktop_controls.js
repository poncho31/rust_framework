export class DesktopControls {

    constructor() {
        // Variable pour suivre le plus haut z-index utilisé
        this.highestZ = 1;

        setInterval(this.updateClock, 1000);
        this.updateClock();

        // Ajout d'un écouteur sur chaque modale et icône de bureau
        document.querySelectorAll('.modal, .desktop_icon').forEach(windowEl => {
            windowEl.addEventListener('mousedown', e => {
                // Si c'est une modale, la mettre en avant
                if (windowEl.classList.contains('modal')) {
                    this.setActiveModal(windowEl.id);
                }
                this.handleDragOrResize(windowEl, e);
            });
        });
          
        document.querySelectorAll('.modal').forEach(windowEl => {
            windowEl.addEventListener('mousemove', e => {
                this.updateModalCursor(windowEl, e);
            });
        });
          
        document.querySelectorAll('.modal .modal_header').forEach(header => {
            header.addEventListener('dblclick', e => {
                const modal = header.closest('.modal');
                this.fullscreenWindow(modal.id);
            });
        });

    }

    // Met la modale active en avant et met à jour l'onglet correspondant
    setActiveModal(modalId) {
        const modal = document.getElementById(modalId);
        this.highestZ++;
        modal.style.zIndex = this.highestZ;
    
        // Ajoute l'effet de blur
        modal.classList.add('blur-effect');
        setTimeout(() => {
            modal.classList.remove('blur-effect');
        }, 1000);
    
        // Met à jour les onglets de la taskbar
        document.querySelectorAll('#taskbarItems .taskbar_item').forEach(item => {
            item.classList.remove('active');
        });
        const taskbarItem = document.getElementById('taskbar_item-' + modalId);
        if (taskbarItem) {
            taskbarItem.classList.add('active');
        }
    }
    

    handleDragOrResize(windowEl, e) {
        e.preventDefault();
        const rect = windowEl.getBoundingClientRect();
        const posX = e.clientX - rect.left;
        const posY = e.clientY - rect.top;
        const threshold = 10;
    
        // Pour une modale, activer le redimensionnement sur tous les bords
        if (windowEl.classList.contains('modal')) {
            const nearLeft   = posX < threshold;
            const nearRight  = posX > windowEl.offsetWidth - threshold;
            const nearTop    = posY < threshold;
            const nearBottom = posY > windowEl.offsetHeight - threshold;
            if (nearLeft || nearRight || nearTop || nearBottom) {
                e.stopPropagation();
                windowEl.classList.add('resizing');
                const startX      = e.clientX,
                      startY      = e.clientY,
                      startWidth  = windowEl.offsetWidth,
                      startHeight = windowEl.offsetHeight,
                      startLeft   = rect.left,
                      startTop    = rect.top;
                      
                const onMouseMoveResize = e => {
                    if (nearRight) {
                        windowEl.style.width = (startWidth + e.clientX - startX) + 'px';
                    }
                    if (nearBottom) {
                        windowEl.style.height = (startHeight + e.clientY - startY) + 'px';
                    }
                    if (nearLeft) {
                        const newWidth = startWidth - (e.clientX - startX);
                        windowEl.style.width = newWidth + 'px';
                        windowEl.style.left  = (startLeft + e.clientX - startX) + 'px';
                    }
                    if (nearTop) {
                        const newHeight = startHeight - (e.clientY - startY);
                        windowEl.style.height = newHeight + 'px';
                        windowEl.style.top    = (startTop + e.clientY - startY) + 'px';
                    }
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
        }
        
        // Pour une icône de bureau ou une modale en dehors de la zone de redimensionnement, activer le déplacement
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
        const threshold = 10; 
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
      
    openWindow($this) {
        let id_modal   = $this.id + '_modal'; 
        let id_content = $this.id + '_content'; 
 
        const w = document.getElementById(id_modal);
        w.style.display = 'block';
        this.addTaskbarItem(id_modal);
      
        const hiddenContent = document.getElementById(id_content);
        if (hiddenContent) {
          w.querySelector('.modal_content').innerHTML = hiddenContent.innerHTML;
          hiddenContent.style.display = 'none'; 
        }
        // Mettre la modale ouverte en avant dès son ouverture
        this.setActiveModal(id_modal);
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
            w.classList.remove('fullscreen');
            w.style.top = '50px';
            w.style.left = '50px';
            w.style.width = '400px';
            w.style.height = '300px';
        } else {
            w.classList.add('fullscreen');
            w.style.top = '0';
            w.style.left = '0';
            w.style.width = '100%';
            w.style.height = 'calc(100% - 40px)';
        }
        // Mettre la modale en avant après le changement de taille
        this.setActiveModal(id);
    }

    removeTaskbarItem(id) {
        const el = document.getElementById('taskbar_item-' + id);
        if (el) {
          el.remove();
        }
    }

    toggle_menu(id) {
        const menu         = document.getElementById(id);
        menu.style.display = menu.style.display === 'block' ? 'none' : 'block';
    }

    // Ajoute un onglet dans la taskbar et gère le clic pour mettre en avant la modale associée
    addTaskbarItem(id) {
        const win = document.getElementById(id);
        const bar = document.getElementById('taskbarItems');
        const taskbar_item_id = 'taskbar_item-' + id;
    
        let taskbar_item = document.getElementById(taskbar_item_id);
        if (!taskbar_item) {
            taskbar_item = document.createElement('div');
            const label = win.querySelector('.modal_header_label').textContent;
    
            taskbar_item.id = taskbar_item_id;
            taskbar_item.className = 'taskbar_item';
            taskbar_item.textContent = label.length > 10 ? label.substring(0, 20) + "..." : label;
            taskbar_item.title = label;
            taskbar_item.style.color = 'white';
            taskbar_item.style.padding = '5px 10px';
            taskbar_item.style.cursor = 'pointer';
    
            taskbar_item.onclick = () => {
                const element = document.getElementById(id);
                // Si la modale n'est pas affichée, l'afficher et la mettre en avant
                if (element.style.display !== 'block') {
                    element.style.display = 'block';
                    this.setActiveModal(id);
                } else {
                    // Si elle est déjà affichée et active, la minimiser
                    if (taskbar_item.classList.contains('active')) {
                        element.style.display = 'none';
                        taskbar_item.classList.remove('active');
                    } else {
                        // Sinon, la mettre en avant
                        this.setActiveModal(id);
                    }
                }
            };
    
            bar.appendChild(taskbar_item);
        }
    }
    
    updateClock() {
        const now = new Date();
        document.getElementById('clock').textContent = now.toLocaleTimeString();
    }
}
