export class DesktopControls {
    constructor() {
        this.highestZ = 1;

        setInterval(this.updateClock, 1000);
        this.updateClock();

        // Ajout d'écouteurs sur chaque modale, icône, widget
        document.querySelectorAll('.modal, .desktop_icon, .desktop_widget').forEach(windowEl => {
            windowEl.addEventListener('mousedown', e => {
                // Met la modale en avant si c'est une modale
                if (windowEl.classList.contains('modal')) {
                    this.setActiveModal(windowEl.id);
                }
                this.handleDragOrResize(windowEl, e);
            });
        });
          
        document.querySelectorAll('.modal, .desktop_widget, .desktop_panel').forEach(windowEl => {
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

    setActiveModal(modalId) {
        const modal = document.getElementById(modalId);
        this.highestZ++;
        modal.style.zIndex = this.highestZ;
    
        modal.classList.add('blur-effect');
        setTimeout(() => {
            modal.classList.remove('blur-effect');
        }, 1000);
    
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

        // 1) Récupère la zone autorisée (rectangle de #desktop)
        const desktopRect = document.getElementById('desktop').getBoundingClientRect();

        const rect      = windowEl.getBoundingClientRect();
        const posX      = e.clientX - rect.left;
        const posY      = e.clientY - rect.top;
        const threshold = 10;
    
        // ----- GESTION REDIMENSIONNEMENT (si class .modal) -----
        if (windowEl.classList.contains('modal')) {
            const nearLeft   = posX < threshold;
            const nearRight  = posX > windowEl.offsetWidth  - threshold;
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
                    // Largeur côté droit
                    if (nearRight) {
                        let newWidth = startWidth + (e.clientX - startX);
                        // Empêche la fenêtre de dépasser le bord droit du desktop
                        const maxWidth = desktopRect.right - startLeft;
                        // Empêche width négative
                        newWidth = Math.max(50, Math.min(newWidth, maxWidth));
                        windowEl.style.width = newWidth + 'px';
                    }
                    // Hauteur côté bas
                    if (nearBottom) {
                        let newHeight = startHeight + (e.clientY - startY);
                        const maxHeight = desktopRect.bottom - startTop;
                        newHeight = Math.max(50, Math.min(newHeight, maxHeight));
                        windowEl.style.height = newHeight + 'px';
                    }
                    // Largeur côté gauche
                    if (nearLeft) {
                        const deltaX   = e.clientX - startX;
                        let newWidth   = startWidth - deltaX;
                        let newLeftVal = startLeft + deltaX;
                        // Empêche la fenêtre de sortir du côté gauche
                        if (newLeftVal < desktopRect.left) {
                            const diff = desktopRect.left - newLeftVal;
                            newLeftVal = desktopRect.left;
                            newWidth   = newWidth - diff; 
                        }
                        newWidth = Math.max(50, newWidth);
                        windowEl.style.width = newWidth + 'px';
                        windowEl.style.left  = newLeftVal + 'px';
                    }
                    // Hauteur côté haut
                    if (nearTop) {
                        const deltaY    = e.clientY - startY;
                        let newHeight   = startHeight - deltaY;
                        let newTopVal   = startTop + deltaY;
                        // Empêche la fenêtre de sortir du haut
                        if (newTopVal < desktopRect.top) {
                            const diff = desktopRect.top - newTopVal;
                            newTopVal = desktopRect.top;
                            newHeight = newHeight - diff;
                        }
                        newHeight = Math.max(50, newHeight);
                        windowEl.style.height = newHeight + 'px';
                        windowEl.style.top    = newTopVal + 'px';
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
        
        // ----- GESTION DÉPLACEMENT (icône ou modale hors zone de resize) -----
        const offsetX = e.clientX - rect.left;
        const offsetY = e.clientY - rect.top;
        let isDragging = false;
        
        const onMouseMove = e => {
            if (!isDragging) {
                if (Math.abs(e.clientX - (rect.left + offsetX)) > 5 ||
                    Math.abs(e.clientY - (rect.top  + offsetY)) > 5) {
                    isDragging = true;
                }
            }
            if (isDragging) {
                // Nouvelle position souhaitée
                let newLeft = e.clientX - offsetX;
                let newTop  = e.clientY - offsetY;

                // Taille courante de l'élément (si redimensionné)
                const currentWidth  = windowEl.offsetWidth;
                const currentHeight = windowEl.offsetHeight;

                // On borne la position dans #desktop
                const minLeft = desktopRect.left;
                const maxLeft = desktopRect.right  - currentWidth;
                const minTop  = desktopRect.top;
                const maxTop  = desktopRect.bottom - currentHeight;

                newLeft = Math.max(minLeft, Math.min(newLeft, maxLeft));
                newTop  = Math.max(minTop,  Math.min(newTop,  maxTop));

                windowEl.style.left = `${newLeft}px`;
                windowEl.style.top  = `${newTop}px`;
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
        const rect       = modal.getBoundingClientRect();
        const threshold  = 10; 
        const nearLeft   = (e.clientX - rect.left) < threshold;
        const nearRight  = (rect.right - e.clientX) < threshold;
        const nearTop    = (e.clientY - rect.top) < threshold;
        const nearBottom = (rect.bottom - e.clientY) < threshold;
        
        if (nearLeft && nearTop) {
            modal.style.cursor = 'nw-resize';
        }
        else if (nearRight && nearTop) {
            modal.style.cursor = 'ne-resize';
        }
        else if (nearLeft && nearBottom) {
            modal.style.cursor = 'sw-resize';
        }
        else if (nearRight && nearBottom) {
            modal.style.cursor = 'se-resize';
        }
        else if (nearLeft || nearRight) {
            modal.style.cursor = 'ew-resize';
        }
        else if (nearTop || nearBottom) {
            modal.style.cursor = 'ns-resize';
        } 
        else {
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
        this.setActiveModal(id);
    }

    removeTaskbarItem(id) {
        const el = document.getElementById('taskbar_item-' + id);
        if (el) el.remove();
    }

    toggle_menu(id) {
        const menu         = document.getElementById(id);
        menu.style.display = menu.style.display === 'block' ? 'none' : 'block';
    }

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
                if (element.style.display !== 'block') {
                    element.style.display = 'block';
                    this.setActiveModal(id);
                } else {
                    if (taskbar_item.classList.contains('active')) {
                        element.style.display = 'none';
                        taskbar_item.classList.remove('active');
                    } else {
                        this.setActiveModal(id);
                    }
                }
            };
    
            bar.appendChild(taskbar_item);
        }
    }
    
    updateClock() {
        const now = new Date();
        const clocks = document.querySelectorAll('.desktop_clock');
        if (clocks.length === 0) {
            console.warn("Aucun élément avec la classe .desktop_clock n'a été trouvé.");
        }
        clocks.forEach(el => {
            if (el !== null) {
                el.textContent = now.toLocaleTimeString();
            }
        });
    }
}
