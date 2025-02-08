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
      // Pas d'input créé d'avance
      this.unifiedInput = null;
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
        if (e.target.closest('#file_modal') || e.target.closest('#folder_modal')) return;
        this.handleDrop(e);
      });
  
      // Au clic, afficher une modal de choix unique entre "Fichier" et "Dossier"
      this.dropZone.addEventListener('click', e => {
        e.preventDefault();
        e.stopPropagation();
        this.showImportChoiceModal(choice => {
          // Créer l'input unified s'il n'existe pas encore
          if (!this.unifiedInput) {
            this.unifiedInput = document.createElement('input');
            this.unifiedInput.type = 'file';
            this.unifiedInput.style.display = 'none';
            // Gérer le changement de sélection
            this.unifiedInput.addEventListener('change', event => {
              const files = event.target.files;
              if (!files.length) return;
              // S'il y a plusieurs fichiers (que ce soit via folder ou sélection multiple), afficher la liste
              if (files.length > 1) {
                this.showFolderImportModal(files);
              } else {
                const file = files[0];
                const category = this.getFileCategory(file);
                if (category) {
                  this.readFile(file, category);
                } else {
                  alert('Type de fichier non supporté.');
                }
              }
              // Réinitialiser l'input pour permettre une nouvelle sélection ultérieure
              this.unifiedInput.value = "";
            });
            document.body.appendChild(this.unifiedInput);
          }
          // Selon le choix, ajuster les attributs de l'input
          if (choice === 'folder') {
            this.unifiedInput.setAttribute('webkitdirectory', '');
            // Le choix de dossier renvoie toujours plusieurs fichiers (contenus du dossier)
            this.unifiedInput.removeAttribute('multiple');
          } else if (choice === 'file') {
            this.unifiedInput.removeAttribute('webkitdirectory');
            this.unifiedInput.setAttribute('multiple', ''); // autoriser la sélection multiple
          }
          // Déclencher l'ouverture de la fenêtre de sélection
          this.unifiedInput.click();
        });
      });
    }
  
    /**
     * Affiche une modal de choix d'importation avec deux boutons ("Fichier" et "Dossier").
     * Le callback reçoit 'file' ou 'folder'.
     */
    showImportChoiceModal(callback) {
      let modal = document.getElementById('import_choice_modal');
      if (!modal) {
        modal = document.createElement('div');
        modal.id = 'import_choice_modal';
        modal.style.position = 'fixed';
        modal.style.top = '0';
        modal.style.left = '0';
        modal.style.width = '100%';
        modal.style.height = '100%';
        modal.style.backgroundColor = 'rgba(0,0,0,0.8)';
        modal.style.display = 'flex';
        modal.style.alignItems = 'center';
        modal.style.justifyContent = 'center';
        modal.style.zIndex = '1100';
        modal.addEventListener('click', e => e.stopPropagation());
        document.body.appendChild(modal);
      }
      modal.innerHTML = '';
  
      const container = document.createElement('div');
      container.style.backgroundColor = '#fff';
      container.style.padding = '20px';
      container.style.borderRadius = '5px';
      container.style.textAlign = 'center';
      container.style.minWidth = '300px';
  
      const title = document.createElement('h3');
      title.textContent = 'Choisissez l\'importation';
      container.appendChild(title);
  
      const btnContainer = document.createElement('div');
      btnContainer.style.display = 'flex';
      btnContainer.style.justifyContent = 'space-around';
      btnContainer.style.marginTop = '20px';
  
      const fileBtn = document.createElement('button');
      fileBtn.textContent = 'Fichier';
      fileBtn.style.padding = '10px 20px';
      fileBtn.style.marginRight = '10px';
      fileBtn.addEventListener('click', e => {
        e.stopPropagation();
        modal.style.display = 'none';
        callback('file');
      });
  
      const folderBtn = document.createElement('button');
      folderBtn.textContent = 'Dossier';
      folderBtn.style.padding = '10px 20px';
      folderBtn.addEventListener('click', e => {
        e.stopPropagation();
        modal.style.display = 'none';
        callback('folder');
      });
  
      btnContainer.appendChild(fileBtn);
      btnContainer.appendChild(folderBtn);
      container.appendChild(btnContainer);
      modal.appendChild(container);
      modal.style.display = 'flex';
    }
  
    /**
     * Détermine la catégorie du fichier.
     * Retourne 'image', 'pdf' ou 'text' selon le type MIME ou l'extension.
     */
    getFileCategory(file) {
      if (file.type.startsWith('image/')) {
        return 'image';
      } else if (file.type === 'application/pdf') {
        return 'pdf';
      } else if (file.type.startsWith('text/')) {
        return 'text';
      } else {
        const ext = file.name.split('.').pop().toLowerCase();
        const codeExtensions = ['php', 'json', 'rs', 'js', 'ts', 'html', 'css', 'md', 'rtf'];
        if (codeExtensions.includes(ext)) {
          return 'text';
        }
      }
      return null;
    }
  
    handleDrop(e) {
      if (e.dataTransfer.items) {
        const items = e.dataTransfer.items;
        for (let i = 0; i < items.length; i++) {
          const item = items[i];
          if (item.kind === 'file') {
            const entry = item.webkitGetAsEntry && item.webkitGetAsEntry();
            if (entry) {
              if (entry.isDirectory) {
                this.readDirectory(entry);
              } else {
                const file = item.getAsFile();
                const category = this.getFileCategory(file);
                if (category) {
                  this.readFile(file, category);
                } else {
                  alert('Type de fichier non supporté.');
                }
              }
            }
          }
        }
      } else {
        const files = e.dataTransfer.files;
        if (!files.length) return;
        const file = files[0];
        const category = this.getFileCategory(file);
        if (category) {
          this.readFile(file, category);
        } else {
          alert('Type de fichier non supporté.');
        }
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
  
    // Affiche une modal listant les fichiers importés depuis un dossier (via le choix dans l'input unifié)
    showFolderImportModal(files) {
      let modal = document.getElementById('folder_modal');
      if (!modal) {
        modal = document.createElement('div');
        modal.id = 'folder_modal';
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
      container.style.maxWidth = '90vw';
      container.style.maxHeight = '80vh';
      container.style.overflow = 'auto';
  
      const title = document.createElement('h2');
      title.textContent = 'Fichiers importés';
      container.appendChild(title);
  
      const list = document.createElement('ul');
      for (let i = 0; i < files.length; i++) {
        const li = document.createElement('li');
        li.textContent = files[i].webkitRelativePath || files[i].name;
        list.appendChild(li);
      }
      container.appendChild(list);
  
      container.addEventListener('click', e => e.stopPropagation());
      modal.appendChild(container);
      modal.style.display = 'flex';
    }
  
    // Méthode pour lire un dossier déposé via drag & drop
    readDirectory(directoryEntry) {
      const reader = directoryEntry.createReader();
      let allEntries = [];
      const readEntries = () => {
        reader.readEntries(entries => {
          if (entries.length) {
            allEntries = allEntries.concat(entries);
            readEntries();
          } else {
            this.showDirectoryModal(directoryEntry.name, allEntries);
          }
        }, error => {
          console.error("Erreur lors de la lecture du dossier :", error);
        });
      };
      readEntries();
    }
  
    // Affiche le contenu du dossier dans une modale (via drag & drop)
    showDirectoryModal(dirName, entries) {
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
      container.style.maxWidth = '90vw';
      container.style.maxHeight = '80vh';
      container.style.overflow = 'auto';
  
      const title = document.createElement('h2');
      title.setAttribute("style", "color:black;");
      title.textContent = `Contenu du dossier : ${dirName}`;
      container.appendChild(title);
  
      const list = document.createElement('ul');
      entries.forEach(entry => {
        const li = document.createElement('li');
        li.setAttribute("style", "color:black;");

        li.textContent =  " - " + entry.name + (entry.isDirectory ? ' (dossier)' : '');
        list.appendChild(li);
      });
      container.appendChild(list);
  
      container.addEventListener('click', e => e.stopPropagation());
      modal.appendChild(container);
      modal.style.display = 'flex';
    }
  }
  