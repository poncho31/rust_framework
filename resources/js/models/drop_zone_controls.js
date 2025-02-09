export class DropZoneControls {
  constructor(dropZoneId = 'desktop_drop_zone') {
      // Empêche le comportement par défaut du drag & drop globalement
      document.addEventListener('dragover', e => e.preventDefault());
      document.addEventListener('drop',     e => e.preventDefault());

      this.dropZone = document.getElementById(dropZoneId);
      if (!this.dropZone) {
          console.error(`L'élément avec l'id "${dropZoneId}" est introuvable.`);
          return;
      }

      // Bind des méthodes pour garantir le contexte
      this.initUnifiedInput       = this.initUnifiedInput.bind(this);
      this.initEvents             = this.initEvents.bind(this);
      this.getFileCategory        = this.getFileCategory.bind(this);
      this.readFile               = this.readFile.bind(this);
      this.handleDrop             = this.handleDrop.bind(this);
      this.showFolderImportModal  = this.showFolderImportModal.bind(this);
      this.readDirectoryRecursive = this.readDirectoryRecursive.bind(this);
      this.readDirectory          = this.readDirectory.bind(this);
      this.showDirectoryModal     = this.showDirectoryModal.bind(this);

      this.unifiedInput = null;
      this.initUnifiedInput();
      this.initEvents();
  }

  initUnifiedInput() {
      if (!this.unifiedInput) {
          this.unifiedInput             = document.createElement('input');
          this.unifiedInput.type        = 'file';
          // La classe "hidden" est définie en CSS pour masquer l'élément
          this.unifiedInput.classList.add('hidden');
          this.unifiedInput.addEventListener('change', event => {
              const files = event.target.files;
              if (!files.length) return;
              if (files.length > 1) {
                  // Si plusieurs fichiers, afficher la modal d'importation de dossier
                  this.showFolderImportModal(files);
              } else {
                  const file     = files[0];
                  const category = this.getFileCategory(file);
                  if (category) {
                      this.readFile(file, category);
                  } else {
                      alert('Type de fichier non supporté.');
                  }
              }
              this.unifiedInput.value = "";
          });
          document.body.appendChild(this.unifiedInput);
      }
  }

  initEvents() {
      // Gestion du drag & drop sur la zone
      this.dropZone.addEventListener('dragover', e => {
          e.preventDefault();
          e.stopPropagation();
      });
      this.dropZone.addEventListener('drop', e => {
          e.preventDefault();
          e.stopPropagation();
          if (
              e.target.closest('#file_modal')    ||
              e.target.closest('#folder_modal')  ||
              e.target.closest('.directory_modal')
          ) return;
          this.handleDrop(e);
      });
      
      // Au clic, afficher la modal de choix d'importation
      this.dropZone.addEventListener('click', e => {
          e.preventDefault();
          e.stopPropagation();
          this.showImportChoiceModal(choice => {
              if (choice === 'folder') {
                  this.unifiedInput.setAttribute('webkitdirectory', '');
                  this.unifiedInput.removeAttribute('multiple');
              } else if (choice === 'file') {
                  this.unifiedInput.removeAttribute('webkitdirectory');
                  this.unifiedInput.setAttribute('multiple', '');
              }
              this.unifiedInput.click();
          });
      });
  }

  /* Création & Affichage des modales en calquant la structure d'exemple */
  createModal(id) {
      let modal = document.getElementById(id);
      if (!modal) {
          modal              = document.createElement('div');
          modal.id           = id;
          modal.classList.add("modal");
          modal.classList.add("box");
          modal.style.display = "none";
          const desktop      = document.getElementById('desktop') || document.body;
          desktop.appendChild(modal);
      }
      // Réinitialise le contenu tout en conservant la structure de base
      modal.innerHTML = "";
      const resizeHandle = document.createElement('div');
      resizeHandle.className = "resize-handle";
      modal.appendChild(resizeHandle);
      return modal;
  }

  showModal(modalId, header, content) {
      const modal = this.createModal(modalId);
      if (header) {
          modal.appendChild(header);
      }
      if (content) {
          modal.appendChild(content);
      }
      modal.style.display = "block";
  }

  hideModal(modalId) {
      const modal = this.createModal(modalId);
      modal.style.display = "none";
  }

  createContainer() {
      const container = document.createElement('div');
      container.classList.add('container');
      container.addEventListener('click', e => e.stopPropagation());
      return container;
  }

  /* ------------------------------
     Modale de choix d'importation
  ------------------------------ */
  showImportChoiceModal(callback) {
      const modalId = 'import_choice_modal';

      // Création du header
      const header = document.createElement('div');
      header.classList.add("modal_header");
      const headerLabel = document.createElement('span');
      headerLabel.classList.add("modal_header_label");
      headerLabel.textContent = "Choisissez l'importation";
      header.appendChild(headerLabel);
      const headerActions = document.createElement('div');
      headerActions.classList.add("modal_header_actions");
      const closeBtn = document.createElement('button');
      closeBtn.classList.add("close-btn");
      closeBtn.title = "Fermer";
      closeBtn.innerHTML = "&#x2716;";
      closeBtn.addEventListener('click', e => {
          e.stopPropagation();
          this.hideModal(modalId);
      });
      headerActions.appendChild(closeBtn);
      header.appendChild(headerActions);

      // Création du contenu avec les boutons Bulma
      const content = document.createElement('div');
      content.classList.add("modal_content");
      const btnContainer = document.createElement('div');
      btnContainer.classList.add("columns", "is-mobile", "is-multiline");
      const colFile = document.createElement('div');
      colFile.classList.add("column", "is-half");
      const fileBtn = document.createElement('button');
      fileBtn.classList.add("button", "is-primary", "is-fullwidth");
      fileBtn.textContent = "Fichier";
      fileBtn.addEventListener('click', e => {
          e.stopPropagation();
          this.hideModal(modalId);
          callback('file');
      });
      colFile.appendChild(fileBtn);
      btnContainer.appendChild(colFile);
      const colFolder = document.createElement('div');
      colFolder.classList.add("column", "is-half");
      const folderBtn = document.createElement('button');
      folderBtn.classList.add("button", "is-info", "is-fullwidth");
      folderBtn.textContent = "Dossier";
      folderBtn.addEventListener('click', e => {
          e.stopPropagation();
          this.hideModal(modalId);
          callback('folder');
      });
      colFolder.appendChild(folderBtn);
      btnContainer.appendChild(colFolder);
      content.appendChild(btnContainer);
      content.style.maxWidth = "25vw";

      this.showModal(modalId, header, content);
  }

  /* ------------------------------
     Modale d'affichage d'un fichier
  ------------------------------ */
  showFileModal(content, type, file) {
      const header = document.createElement('div');
      header.classList.add("modal_header");
      const headerLabel = document.createElement('span');
      headerLabel.classList.add("modal_header_label");
      headerLabel.textContent = file.name || "Fichier";
      header.appendChild(headerLabel);
      const headerActions = document.createElement('div');
      headerActions.classList.add("modal_header_actions");
      const closeBtn = document.createElement('button');
      closeBtn.classList.add("close-btn");
      closeBtn.title = "Fermer";
      closeBtn.innerHTML = "&#x2716;";
      closeBtn.addEventListener('click', e => {
          e.stopPropagation();
          this.hideModal('file_modal');
      });
      headerActions.appendChild(closeBtn);
      header.appendChild(headerActions);

      const contentDiv = document.createElement('div');
      contentDiv.classList.add("modal_content");
      if (type === 'image') {
          const img = document.createElement('img');
          img.src = content;
          img.alt = "Image affichée";
          img.classList.add("modal_image");
          contentDiv.appendChild(img);
      } else if (type === 'pdf') {
          const iframe = document.createElement('iframe');
          iframe.src = content;
          iframe.classList.add("modal_iframe");
          contentDiv.appendChild(iframe);
      } else if (type === 'text') {
          const pre = document.createElement('pre');
          pre.classList.add("modal_text");
          pre.textContent = content;
          contentDiv.appendChild(pre);
      }
      this.showModal('file_modal', header, contentDiv);
  }

  /* ------------------------------
     Modale d'affichage des fichiers importés
  ------------------------------ */
  showFolderImportModal(files) {
      const header = document.createElement('div');
      header.classList.add("modal_header");
      const headerLabel = document.createElement('span');
      headerLabel.classList.add("modal_header_label");
      headerLabel.textContent = "Fichiers importés";
      header.appendChild(headerLabel);
      const headerActions = document.createElement('div');
      headerActions.classList.add("modal_header_actions");
      const closeBtn = document.createElement('button');
      closeBtn.classList.add("close-btn");
      closeBtn.title = "Fermer";
      closeBtn.innerHTML = "&#x2716;";
      closeBtn.addEventListener('click', e => {
          e.stopPropagation();
          this.hideModal('folder_modal');
      });
      headerActions.appendChild(closeBtn);
      header.appendChild(headerActions);

      const contentDiv = document.createElement('div');
      contentDiv.classList.add("modal_content");
      const grid = document.createElement('div');
      grid.classList.add("columns", "is-multiline");
      for (const file of files) {
          const category = this.getFileCategory(file);
          const column   = document.createElement('div');
          column.classList.add("column", "is-one-quarter");
          const card = document.createElement('div');
          card.classList.add("card");
          card.style.cursor = "pointer";
          const cardImage = document.createElement('div');
          cardImage.classList.add("card-image");
          const figure = document.createElement('figure');
          figure.classList.add("image", "is-128x128");
          const fileImg = document.createElement('img');
          fileImg.src = "/images/icons/file_icon.png";
          fileImg.alt = file.name;
          figure.appendChild(fileImg);
          cardImage.appendChild(figure);
          const cardContent = document.createElement('div');
          cardContent.classList.add("card-content");
          const fileLabel = document.createElement('p');
          fileLabel.classList.add("title", "is-6");
          fileLabel.textContent = file.name;
          cardContent.appendChild(fileLabel);
          card.appendChild(cardImage);
          card.appendChild(cardContent);
          card.addEventListener('click', e => {
              e.stopPropagation();
              if (category) {
                  this.readFile(file, category);
              } else {
                  alert("Type de fichier non supporté.");
              }
          });
          column.appendChild(card);
          grid.appendChild(column);
      }
      contentDiv.appendChild(grid);
      this.showModal('folder_modal', header, contentDiv);
  }

  /* ------------------------------
     Gestion récursive des dossiers
  ------------------------------ */
  readDirectoryRecursive(directoryEntry) {
      return new Promise((resolve, reject) => {
          const files       = [];
          const directories = [];
          let pending       = 0;
          function traverse(entry, path = "") {
              if (entry.isFile) {
                  pending++;
                  entry.file(file => {
                      file.fullPath = path + file.name;
                      files.push(file);
                      pending--;
                      if (pending === 0) resolve({ files, directories });
                  }, error => {
                      console.error("Erreur lors de la lecture du fichier :", error);
                      pending--;
                      if (pending === 0) resolve({ files, directories });
                  });
              } else if (entry.isDirectory) {
                  directories.push({ name: path + entry.name, entry });
                  pending++;
                  const dirReader = entry.createReader();
                  dirReader.readEntries(entries => {
                      pending--;
                      entries.forEach(child => traverse(child, path + entry.name + "/"));
                      if (pending === 0) resolve({ files, directories });
                  }, error => {
                      console.error("Erreur lors de la lecture du dossier :", error);
                      pending--;
                      if (pending === 0) resolve({ files, directories });
                  });
              }
          }
          traverse(directoryEntry);
          if (pending === 0) resolve({ files, directories });
      });
  }

  readDirectory(directoryEntry, modalId) {
      modalId = modalId || ('directory_modal_' + Date.now());
      this.readDirectoryRecursive(directoryEntry)
          .then(({ files, directories }) => {
              if (files.length > 0 || directories.length > 0) {
                  this.showDirectoryModal(modalId, directoryEntry.name, files, directories);
              } else {
                  alert("Aucun fichier trouvé dans le dossier.");
              }
          })
          .catch(error => {
              console.error("Erreur lors de la lecture récursive du dossier :", error);
          });
  }

  /* ------------------------------
     Modale d'affichage d'un dossier
  ------------------------------ */
  showDirectoryModal(modalId, dirName, files, directories) {
      const header = document.createElement('div');
      header.classList.add(
          "modal_header",
          "has-background-grey-dark",
          "has-text-white",
          "is-flex",
          "is-justify-content-space-between",
          "is-align-items-center",
          "p-2"
      );
      const headerLabel = document.createElement('span');
      headerLabel.classList.add("title", "is-5");
      headerLabel.textContent = `Contenu du dossier : ${dirName}`;
      header.appendChild(headerLabel);
      const headerActions = document.createElement('div');
      headerActions.classList.add("modal_header_actions");
      const closeBtn = document.createElement('button');
      closeBtn.classList.add("delete");
      closeBtn.title = "Fermer";
      closeBtn.addEventListener('click', e => {
          e.stopPropagation();
          this.hideModal(modalId);
      });
      headerActions.appendChild(closeBtn);
      header.appendChild(headerActions);

      const contentDiv = document.createElement('div');
      contentDiv.classList.add("modal_content");

      if (directories.length > 0) {
          const dirTitle = document.createElement('h3');
          dirTitle.textContent = "Dossiers";
          dirTitle.classList.add("title", "is-4", "mt-4");
          contentDiv.appendChild(dirTitle);
          
          const dirColumns = document.createElement('div');
          dirColumns.classList.add("columns", "is-multiline");
          directories.forEach(dir => {
              const column = document.createElement('div');
              column.classList.add("column", "is-one-quarter");
              const card = document.createElement('div');
              card.classList.add("card");
              card.style.cursor = "pointer";
              const cardImage = document.createElement('div');
              cardImage.classList.add("card-image");
              const figure = document.createElement('figure');
              figure.classList.add("image", "is-128x128");
              const folderIcon = document.createElement('img');
              folderIcon.src = "/images/icons/folder_icon.png";
              folderIcon.alt = dir.name;
              figure.appendChild(folderIcon);
              cardImage.appendChild(figure);
              const cardContent = document.createElement('div');
              cardContent.classList.add("card-content");
              const folderLabel = document.createElement('p');
              folderLabel.classList.add("title", "is-6");
              folderLabel.textContent = dir.name;
              cardContent.appendChild(folderLabel);
              card.appendChild(cardImage);
              card.appendChild(cardContent);
              card.addEventListener('click', e => {
                  e.stopPropagation();
                  this.readDirectory(dir.entry, "directory_modal_" + Date.now());
              });
              column.appendChild(card);
              dirColumns.appendChild(column);
          });
          contentDiv.appendChild(dirColumns);
      }

      if (files.length > 0) {
          const fileTitle = document.createElement('h3');
          fileTitle.textContent = "Fichiers";
          fileTitle.classList.add("title", "is-4", "mt-4");
          contentDiv.appendChild(fileTitle);
          const fileColumns = document.createElement('div');
          fileColumns.classList.add("columns", "is-multiline");
          files.forEach(file => {
              const category = this.getFileCategory(file);
              const column   = document.createElement('div');
              column.classList.add("column", "is-one-quarter");
              const card = document.createElement('div');
              card.classList.add("card");
              card.style.cursor = "pointer";
              const cardImage = document.createElement('div');
              cardImage.classList.add("card-image");
              const figure = document.createElement('figure');
              figure.classList.add("image", "is-128x128");
              const fileIcon = document.createElement('img');
              fileIcon.src = "/images/icons/file_icon.png";
              fileIcon.alt = file.name;
              figure.appendChild(fileIcon);
              cardImage.appendChild(figure);
              const cardContent = document.createElement('div');
              cardContent.classList.add("card-content");
              const fileLabel = document.createElement('p');
              fileLabel.classList.add("title", "is-6");
              fileLabel.textContent = file.name;
              cardContent.appendChild(fileLabel);
              card.appendChild(cardImage);
              card.appendChild(cardContent);
              card.addEventListener('click', e => {
                  e.stopPropagation();
                  if (category) {
                      this.readFile(file, category);
                  } else {
                      alert("Type de fichier non supporté.");
                  }
              });
              column.appendChild(card);
              fileColumns.appendChild(column);
          });
          contentDiv.appendChild(fileColumns);
      }

      this.showModal(modalId, header, contentDiv);
  }

  /* ----------------------------------------------------
     Gestion récursive des dossiers
  ---------------------------------------------------- */
  readDirectoryRecursive(directoryEntry) {
      return new Promise((resolve, reject) => {
          const files       = [];
          const directories = [];
          let pending       = 0;
          function traverse(entry, path = "") {
              if (entry.isFile) {
                  pending++;
                  entry.file(file => {
                      file.fullPath = path + file.name;
                      files.push(file);
                      pending--;
                      if (pending === 0) resolve({ files, directories });
                  }, error => {
                      console.error("Erreur lors de la lecture du fichier :", error);
                      pending--;
                      if (pending === 0) resolve({ files, directories });
                  });
              } else if (entry.isDirectory) {
                  directories.push({ name: path + entry.name, entry });
                  pending++;
                  const dirReader = entry.createReader();
                  dirReader.readEntries(entries => {
                      pending--;
                      entries.forEach(child => traverse(child, path + entry.name + "/"));
                      if (pending === 0) resolve({ files, directories });
                  }, error => {
                      console.error("Erreur lors de la lecture du dossier :", error);
                      pending--;
                      if (pending === 0) resolve({ files, directories });
                  });
              }
          }
          traverse(directoryEntry);
          if (pending === 0) resolve({ files, directories });
      });
  }

  readDirectory(directoryEntry, modalId) {
      modalId = modalId || ('directory_modal_' + Date.now());
      this.readDirectoryRecursive(directoryEntry)
          .then(({ files, directories }) => {
              if (files.length > 0 || directories.length > 0) {
                  this.showDirectoryModal(modalId, directoryEntry.name, files, directories);
              } else {
                  alert("Aucun fichier trouvé dans le dossier.");
              }
          })
          .catch(error => {
              console.error("Erreur lors de la lecture récursive du dossier :", error);
          });
  }

  
  getFileCategory(file) {
    if (file.type.startsWith('image/')) return 'image';
    else if (file.type === 'application/pdf') return 'pdf';
    else if (file.type.startsWith('text/')) return 'text';
    else {
      const ext = file.name.split('.').pop().toLowerCase();
      const codeExtensions = ['php', 'json', 'rs', 'js', 'ts', 'html', 'css', 'md', 'rtf', 'xlsx', 'doc'];
      if (codeExtensions.includes(ext)) return 'text';
    }
    return null;
  }

    readFile(file, type) {
    const reader = new FileReader();
    reader.onload = event => {
      const content = event.target.result;
      this.showFileModal(content, type, file);
    };
    if (type === 'text') reader.readAsText(file);
    else reader.readAsDataURL(file);
  }



  handleDrop(e) {
    if (e.dataTransfer.items) {
      for (let i = 0; i < e.dataTransfer.items.length; i++) {
        const item = e.dataTransfer.items[i];
        if (item.kind === 'file') {
          const entry = item.webkitGetAsEntry && item.webkitGetAsEntry();
          if (entry) {
            if (entry.isDirectory) {
              this.readDirectory(entry);
            } else {
              const file = item.getAsFile();
              const category = this.getFileCategory(file);
              if (category) this.readFile(file, category);
              else alert('Type de fichier non supporté.');
            }
          }
        }
      }
    } else {
      const files = e.dataTransfer.files;
      if (!files.length) return;
      const file = files[0];
      const category = this.getFileCategory(file);
      if (category) this.readFile(file, category);
      else alert('Type de fichier non supporté.');
    }
  }


}

