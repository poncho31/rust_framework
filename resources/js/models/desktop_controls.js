export class DesktopControls {
    constructor() {
        this.highestZ = 1;

        setInterval(this.updateClock, 1000);
        this.updateClock();

        // Ajout d'écouteurs sur chaque modale, icône, widget
        document.querySelectorAll('.modal, .desktop_icon, .desktop_widget, .icon').forEach(windowEl => {
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
    
        let container, containerRect;
        if (windowEl.classList.contains('icon')) {
            // Pour les icônes, utiliser le conteneur des icônes
            container = document.getElementById('desktop_icons');
            containerRect = container.getBoundingClientRect();
    
            // Si l'icône est en position static, la convertir en absolute
            if (getComputedStyle(windowEl).position === 'static') {
                const iconRect = windowEl.getBoundingClientRect();
                windowEl.style.position = 'absolute';
                // Calculer la position relative au conteneur
                windowEl.style.left = (iconRect.left - containerRect.left) + 'px';
                windowEl.style.top  = (iconRect.top - containerRect.top) + 'px';
            }
        } else {
            container = document.getElementById('body');
            containerRect = container.getBoundingClientRect();
        }
    
        // Pour les icônes, la zone de référence est désormais #desktop_icons
        const play_zone = windowEl.classList.contains('icon') ? 'desktop_icons' : 'body';
        const desktopRect = document.getElementById(play_zone).getBoundingClientRect();
    
        const rect = windowEl.getBoundingClientRect();
        const posX = e.clientX - rect.left;
        const posY = e.clientY - rect.top;
        const threshold = 10;
    
        // ---- GESTION REDIMENSIONNEMENT (pour les modales) ----
        if (windowEl.classList.contains('modal')) {
            const nearLeft   = posX < threshold;
            const nearRight  = posX > windowEl.offsetWidth - threshold;
            const nearTop    = posY < threshold;
            const nearBottom = posY > windowEl.offsetHeight - threshold;
            if (nearLeft || nearRight || nearTop || nearBottom) {
                e.stopPropagation();
                windowEl.classList.add('resizing');
    
                const startX = e.clientX,
                      startY = e.clientY,
                      startWidth = windowEl.offsetWidth,
                      startHeight = windowEl.offsetHeight,
                      startLeft = rect.left,
                      startTop = rect.top;
    
                const onMouseMoveResize = e => {
                    if (nearRight) {
                        let newWidth = startWidth + (e.clientX - startX);
                        const maxWidth = desktopRect.right - startLeft;
                        newWidth = Math.max(50, Math.min(newWidth, maxWidth));
                        windowEl.style.width = newWidth + 'px';
                    }
                    if (nearBottom) {
                        let newHeight = startHeight + (e.clientY - startY);
                        const maxHeight = desktopRect.bottom - startTop;
                        newHeight = Math.max(50, Math.min(newHeight, maxHeight));
                        windowEl.style.height = newHeight + 'px';
                    }
                    if (nearLeft) {
                        const deltaX = e.clientX - startX;
                        let newWidth = startWidth - deltaX;
                        let newLeftVal = startLeft + deltaX;
                        if (newLeftVal < desktopRect.left) {
                            const diff = desktopRect.left - newLeftVal;
                            newLeftVal = desktopRect.left;
                            newWidth -= diff;
                        }
                        newWidth = Math.max(50, newWidth);
                        windowEl.style.width = newWidth + 'px';
                        windowEl.style.left = newLeftVal + 'px';
                    }
                    if (nearTop) {
                        const deltaY = e.clientY - startY;
                        let newHeight = startHeight - deltaY;
                        let newTopVal = startTop + deltaY;
                        if (newTopVal < desktopRect.top) {
                            const diff = desktopRect.top - newTopVal;
                            newTopVal = desktopRect.top;
                            newHeight -= diff;
                        }
                        newHeight = Math.max(50, newHeight);
                        windowEl.style.height = newHeight + 'px';
                        windowEl.style.top = newTopVal + 'px';
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
    
        // ---- GESTION DÉPLACEMENT ----
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
                let newLeft, newTop;
                if (windowEl.classList.contains('icon')) {
                    // Calculer la nouvelle position relative au conteneur
                    newLeft = e.clientX - containerRect.left - offsetX;
                    newTop  = e.clientY - containerRect.top - offsetY;
    
                    const currentWidth = windowEl.offsetWidth;
                    const currentHeight = windowEl.offsetHeight;
                    newLeft = Math.max(0, Math.min(newLeft, containerRect.width - currentWidth));
                    newTop  = Math.max(0, Math.min(newTop, containerRect.height - currentHeight));
                } else {
                    newLeft = e.clientX - offsetX;
                    newTop  = e.clientY - offsetY;
                    const currentWidth = windowEl.offsetWidth;
                    const currentHeight = windowEl.offsetHeight;
                    const minLeft = desktopRect.left;
                    const maxLeft = desktopRect.right - currentWidth;
                    const minTop = desktopRect.top;
                    const maxTop = desktopRect.bottom - currentHeight;
                    newLeft = Math.max(minLeft, Math.min(newLeft, maxLeft));
                    newTop  = Math.max(minTop, Math.min(newTop, maxTop));
                }
                windowEl.style.left = newLeft + 'px';
                windowEl.style.top = newTop + 'px';
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
        const nearLeft = (e.clientX - rect.left) < threshold;
        const nearRight = (rect.right - e.clientX) < threshold;
        const nearTop = (e.clientY - rect.top) < threshold;
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
        let id_modal = $this.id + '_modal';
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
        const desktop = document.getElementById('desktop');
        const desktopRect = desktop.getBoundingClientRect();
    
        if (w.classList.contains('fullscreen')) {
            w.classList.remove('fullscreen');
            w.style.top = '50px';
            w.style.left = '50px';
            w.style.width = (desktopRect.width/2) + 'px';
            w.style.height = (desktopRect.height/2) + 'px';
        } else {
            w.classList.add('fullscreen');
            w.style.top = desktopRect.top + 'px';
            w.style.left = desktopRect.left + 'px';
            w.style.width = desktopRect.width + 'px';
            // Optionnel : vous pouvez ajuster la hauteur, ici on prend toute la hauteur de #desktop
            w.style.height = desktopRect.height + 'px';
        }
        this.setActiveModal(id);
    }
    
    
    removeTaskbarItem(id) {
        const el = document.getElementById('taskbar_item-' + id);
        if (el) el.remove();
    }
    
    toggle_menu(id) {
        const menu = document.getElementById(id);
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
    
        // Mise à jour de l'horloge avec l'heure seule et ajout du title (date et heure)
        const clocks = document.querySelectorAll('.desktop_clock');
        if (clocks.length === 0) {
            console.warn("Aucun élément avec la classe .desktop_clock n'a été trouvé.");
        }
        clocks.forEach(el => {
            if (el !== null) {
                el.innerHTML = `
<div >
  <p style="margin: 0; margin-top:5px; line-height: 0.8;">${now.toLocaleTimeString()}</p>
  <small style="margin: 0; line-height: 0.8;">${now.toLocaleDateString('fr-FR', { weekday: 'long', day: 'numeric', month: 'long', year: 'numeric' })}</small>
</div>



                               `;
                el.title       = now.toLocaleDateString() + ' ' + now.toLocaleTimeString();
            }
        });
    }
    
    

    togglePanel() {
        // On masque/affiche le panel
        const panel = document.getElementById('desktop_panel');
        panel.classList.toggle('desktop_panel_hidden');
      }
}
