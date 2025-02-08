export class DropZoneControls {
    constructor(dropZoneId = 'desktop_drop_zone') {
      // Empêcher le comportement drop global
      document.addEventListener('dragover', e => {
        e.preventDefault();
      });
      document.addEventListener('drop', e => {
        e.preventDefault();
      });
  
      this.dropZone = document.getElementById(dropZoneId);
      if (!this.dropZone) {
        console.error(`L'élément avec l'id "${dropZoneId}" est introuvable.`);
        return;
      }
      this.initEvents();
    }
  
    initEvents() {
      // Gestion du drag & drop
      this.dropZone.addEventListener('dragover', e => {
        e.preventDefault();
        e.stopPropagation();
      });
      this.dropZone.addEventListener('drop', e => {
        e.preventDefault();
        e.stopPropagation();
        if (e.target.closest('#file_modal')) return;
        this.handleDrop(e);
      });
      // Gestion du clic pour ouvrir le sélecteur de fichiers
      this.dropZone.addEventListener('click', e => {
        e.preventDefault();
        e.stopPropagation();
        if (!this.fileInput) {
          this.fileInput = document.createElement('input');
          this.fileInput.type = 'file';
          this.fileInput.style.display = 'none';
          this.fileInput.addEventListener('change', event => {
            const files = event.target.files;
            if (!files.length) return;
            const file = files[0];
            if (file.type.startsWith('image/')) {
              this.readFile(file, 'image');
            } else if (file.type === 'application/pdf') {
              this.readFile(file, 'pdf');
            } else if (file.type.startsWith('text/')) {
              this.readFile(file, 'text');
            } else {
              alert('Type de fichier non supporté.');
            }
          });
          document.body.appendChild(this.fileInput);
        }
        this.fileInput.click();
      });
    }
  
    handleDrop(e) {
      const files = e.dataTransfer.files;
      if (!files.length) return;
      const file = files[0];
  
      if (file.type.startsWith('image/')) {
        this.readFile(file, 'image');
      } else if (file.type === 'application/pdf') {
        this.readFile(file, 'pdf');
      } else if (file.type.startsWith('text/')) {
        this.readFile(file, 'text');
      } else {
        alert('Type de fichier non supporté.');
      }
    }
  
    readFile(file, type) {
      const reader = new FileReader();
      reader.onload = event => {
        const content = event.target.result;
        this.showModal(content, type, file);
      };
      if (type === 'text') {
        reader.readAsText(file);
      } else {
        reader.readAsDataURL(file);
      }
    }
  
    showModal(content, type, file) {
      let modal = document.getElementById('file_modal');
      if (!modal) {
        modal = document.createElement('div');
        modal.id = 'file_modal';
        modal.style.position = 'fixed';
        modal.style.top = '0';
        modal.style.left = '0';
        modal.style.width = '100%';
        modal.style.height = '100%';
        modal.style.backgroundColor = 'rgba(0,0,0,0.8)';
        modal.style.display = 'flex';
        modal.style.alignItems = 'center';
        modal.style.justifyContent = 'center';
        modal.style.zIndex = '1000';
        modal.addEventListener('click', () => {
          modal.style.display = 'none';
        });
        document.body.appendChild(modal);
      }
      modal.innerHTML = '';
  
      const container = document.createElement('div');
      container.style.backgroundColor = '#fff';
      container.style.padding = '20px';
      container.style.borderRadius = '5px';
      container.style.display = 'flex';
      container.style.flexDirection = 'column';
      container.style.alignItems = 'center';
  
      if (type === 'image') {
        const img = document.createElement('img');
        img.src = content;
        img.alt = "Image affichée";
        img.style.maxWidth = '90%';
        img.style.maxHeight = '80vh';
        container.appendChild(img);
  
        const btnContainer = document.createElement('div');
        btnContainer.style.marginTop = '10px';
        btnContainer.style.display = 'flex';
        btnContainer.style.gap = '10px';
  
        const saveBtn = document.createElement('a');
        saveBtn.href = content;
        saveBtn.download = file.name || 'image';
        saveBtn.title = 'Sauvegarder';
        saveBtn.style.textDecoration = 'none';
        saveBtn.style.border = 'none';
        saveBtn.style.background = 'none';
        saveBtn.style.cursor = 'pointer';
        saveBtn.innerHTML = `
          <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="#000" viewBox="0 0 16 16">
            <path d="M8 5a.5.5 0 0 1 .5.5v5a.5.5 0 0 1-1 0v-5A.5.5 0 0 1 8 5z"/>
            <path d="M2 1a2 2 0 0 0-2 2v10.293c0 .63.81 1.17 1.38.89l2.906-1.453A2 2 0 0 1 6.5 12h3a2 2 0 0 1 1.214.39l2.906 1.453A1.5 1.5 0 0 0 16 13.293V3a2 2 0 0 0-2-2H2z"/>
          </svg>
        `;
  
        const deleteBtn = document.createElement('button');
        deleteBtn.title = 'Supprimer';
        deleteBtn.style.cursor = 'pointer';
        deleteBtn.style.border = 'none';
        deleteBtn.style.background = 'none';
        deleteBtn.innerHTML = `
          <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="#000" viewBox="0 0 16 16">
            <path d="M5.5 5.5a.5.5 0 0 1 .5-.5H10a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-.5.5H6a.5.5 0 0 1-.5-.5v-6z"/>
            <path fill-rule="evenodd" d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4H1.5a1 1 0 0 1 0-2H5l.5-1h4l.5 1h3a1 1 0 0 1 1 1zM5 4v9a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1V4H5z"/>
          </svg>
        `;
        deleteBtn.addEventListener('click', e => {
          e.stopPropagation();
          modal.style.display = 'none';
        });
  
        btnContainer.appendChild(saveBtn);
        btnContainer.appendChild(deleteBtn);
        container.appendChild(btnContainer);
      } else if (type === 'pdf') {
        const iframe = document.createElement('iframe');
        iframe.src = content;
        iframe.style.width = '90vw';
        iframe.style.height = '80vh';
        container.appendChild(iframe);
      } else if (type === 'text') {
        const pre = document.createElement('pre');
        pre.style.minWidth = '80vw';
        pre.style.maxWidth = '80vw';
        pre.style.minHeight = '70vh';
        pre.style.maxHeight = '70vh';
        pre.style.background = '#fff';
        pre.style.padding = '10px';
        pre.style.overflow = 'auto';
        pre.textContent = content;
        container.appendChild(pre);
      }
  
      container.addEventListener('click', e => e.stopPropagation());
      modal.appendChild(container);
      modal.style.display = 'flex';
    }
  }
  