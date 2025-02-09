/**
 * Classe gérant une zone de dépôt (drag & drop) et l'importation de fichiers/dossiers.
 */
export class DropZoneControls {
  /**
   * Constructeur.
   * @param {string} dropZoneId - L'identifiant de la zone de dépôt (défaut : 'desktop_drop_zone').
   */
  constructor(dropZoneId = 'desktop_drop_zone') {
    // Empêche le comportement global par défaut du drag & drop
    document.addEventListener('dragover', e => e.preventDefault());
    document.addEventListener('drop', e => e.preventDefault());

    this.dropZone = document.getElementById(dropZoneId);
    if (!this.dropZone) {
      console.error(`L'élément avec l'id "${dropZoneId}" est introuvable.`);
      return;
    }
    this.unifiedInput = null;
    this.initUnifiedInput();
    this.initEvents();
  }

  /**
   * Initialise l'input de sélection de fichiers de manière centralisée.
   */
  initUnifiedInput() {
    if (!this.unifiedInput) {
      this.unifiedInput               = document.createElement('input');
      this.unifiedInput.type          = 'file';
      this.unifiedInput.style.display = 'none';
      this.unifiedInput.addEventListener('change', event => {
        const files = event.target.files;
        if (!files.length) return;
        if (files.length > 1) {
          // Plusieurs fichiers : affiche la modal d'importation de dossier
          this.showFolderImportModal(files);
        } else {
          const file = files[0];
          const category = this.getFileCategory(file);
          if (category) this.readFile(file, category);
          else alert('Type de fichier non supporté.');
        }
        this.unifiedInput.value = "";
      });
      document.body.appendChild(this.unifiedInput);
    }
  }

  /**
   * Initialise les événements de la zone de dépôt.
   * - Gère le drag & drop.
   * - Affiche la modal de choix d'importation lors d'un clic.
   */
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
        e.target.closest('#file_modal') ||
        e.target.closest('#folder_modal') ||
        e.target.closest('.directory_modal')
      ) return;
      this.handleDrop(e);
    });
    // Gestion du clic pour choisir le type d'importation
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

  /**
   * Crée (ou réinitialise) une modal.
   * @param {string} id - L'identifiant de la modal.
   * @returns {HTMLElement} La modal créée ou existante.
   */
  createModal(id) {
    let modal = document.getElementById(id);
    if (!modal) {
      modal               = document.createElement('div');
      modal.id            = id;
      modal.style.display = 'none';
      modal.classList.add("desktop_display_file");
      document.getElementById('desktop').appendChild(modal);
    }
    modal.innerHTML = '';
    return modal;
  }

  /**
   * Affiche une modal en se basant sur un header et un contenu.
   * @param {string} modalId - L'identifiant de la modal.
   * @param {HTMLElement|null} header - L'en-tête de la modal.
   * @param {HTMLElement} content - Le contenu de la modal.
   * @param {string} [displayStyle='flex'] - Le style d'affichage de la modal.
   */
  showModal(modalId, header, content, displayStyle = 'flex') {
    const modal = this.createModal(modalId);
    if (header) {
      modal.appendChild(header);
    }
    modal.appendChild(content);
    modal.style.display = displayStyle;
  }

  /**
   * Crée un conteneur interne avec des styles personnalisés.
   * @param {Object} customStyles - Objet de styles CSS.
   * @returns {HTMLElement} Le conteneur créé.
   */
  createContainer(customStyles = {}) {
    const container    = document.createElement('div');
    container.maxWidth = "200px";
    for (const key in customStyles) {
      container.style[key] = customStyles[key];
    }
    container.addEventListener('click', e => e.stopPropagation());
    return container;
  }

  /**
   * Masque la modal spécifiée en utilisant createModal.
   * @param {string} modalId - L'identifiant de la modal.
   */
  hideModal(modalId) {
    const modal = this.createModal(modalId);
    modal.style.display = 'none';
  }

  /**
   * Affiche la modal de choix d'importation (fichier ou dossier) en utilisant createModal et showModal.
   * @param {function} callback - Callback recevant 'file' ou 'folder'.
   */
  showImportChoiceModal(callback) {
    const container = this.createContainer();
    const title     = document.createElement('h3');
    title.textContent = 'Choisissez l\'importation';
    container.appendChild(title);
    container.style.width = "100px";
    container.style.maxWidth = "100px";

    const btnContainer                = document.createElement('div');
    btnContainer.style.display        = 'flex';
    btnContainer.style.justifyContent = 'space-around';
    btnContainer.style.marginTop      = '20px';

    const fileBtn           = document.createElement('button');
    fileBtn.textContent     = 'Fichier';
    fileBtn.style.padding   = '10px 20px';
    fileBtn.addEventListener('click', e => {
      e.stopPropagation();
      this.hideModal('import_choice_modal');
      callback('file');
    });

    const folderBtn         = document.createElement('button');
    folderBtn.textContent   = 'Dossier';
    folderBtn.style.padding = '10px 20px';
    folderBtn.addEventListener('click', e => {
      e.stopPropagation();
      this.hideModal('import_choice_modal');
      callback('folder');
    });

    btnContainer.appendChild(fileBtn);
    btnContainer.appendChild(folderBtn);
    container.appendChild(btnContainer);
    this.showModal('import_choice_modal', null, container, 'flex');
  }


  /**
   * Détermine la catégorie d'un fichier selon son type MIME ou son extension.
   * @param {File} file - Le fichier à évaluer.
   * @returns {string|null} 'image', 'pdf', 'text' ou null si non supporté.
   */
  getFileCategory(file) {
    if (file.type.startsWith('image/'))       return 'image';
    else if (file.type === 'application/pdf') return 'pdf';
    else if (file.type.startsWith('text/'))   return 'text';
    else {
      const ext = file.name.split('.').pop().toLowerCase();
      const codeExtensions = ['php', 'json', 'rs', 'js', 'ts', 'html', 'css', 'md', 'rtf', 'xlsx', 'doc'];
      if (codeExtensions.includes(ext)) return 'text';
    }
    return null;
  }

  /**
   * Gère l'événement de dépôt (drop) sur la zone.
   * Identifie et traite les fichiers/dossiers déposés.
   * @param {DragEvent} e - L'événement de drop.
   */
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
      const file     = files[0];
      const category = this.getFileCategory(file);
      if (category) this.readFile(file, category);
      else alert('Type de fichier non supporté.');
    }
  }

  /**
   * Lit un fichier et affiche son contenu dans une modal.
   * @param {File} file - Le fichier à lire.
   * @param {string} type - La catégorie du fichier ('image', 'pdf', 'text').
   */
  readFile(file, type) {
    const reader = new FileReader();
    reader.onload = event => {
      const content = event.target.result;
      this.showFileModal(content, type, file);
    };
    if (type === 'text') reader.readAsText(file);
    else reader.readAsDataURL(file);
  }

  /**
   * Affiche le contenu d'un fichier dans une modal structurée.
   * @param {string} content - Le contenu du fichier.
   * @param {string} type - La catégorie du fichier.
   * @param {File} file - Le fichier concerné.
   */
  showFileModal(content, type, file) {
    const header = document.createElement('div');
    header.className = 'modal_header';
    const headerLabel       = document.createElement('span');
    headerLabel.className   = 'modal_header_label';
    headerLabel.textContent = file.name || 'Fichier';
    header.appendChild(headerLabel);
    const headerActions     = document.createElement('div');
    headerActions.className = 'modal_header_actions';
    const closeBtn          = document.createElement('div');
    closeBtn.className      = 'close-btn';
    closeBtn.title          = 'Fermer';
    closeBtn.innerHTML      = '&#x2716;';
    closeBtn.addEventListener('click', e => {
      e.stopPropagation();
      document.getElementById('file_modal').style.display = 'none';
    });
    headerActions.appendChild(closeBtn);
    header.appendChild(headerActions);

    const contentDiv = document.createElement('div');
    contentDiv.className = 'modal_content';
    if (type === 'image') {
      const img = document.createElement('img');
      img.src = content;
      img.alt = "Image affichée";
      img.style.maxWidth = '90%';
      img.style.maxHeight = '80vh';
      contentDiv.appendChild(img);
    } else if (type === 'pdf') {
      const iframe = document.createElement('iframe');
      iframe.src = content;
      iframe.style.width = '90vw';
      iframe.style.height = '80vh';
      contentDiv.appendChild(iframe);
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
      contentDiv.appendChild(pre);
    }
    this.showModal('file_modal', header, contentDiv, 'flex');
  }

  /**
   * Affiche une modal listant les fichiers importés depuis un dossier.
   * @param {FileList} files - La liste des fichiers importés.
   */
  showFolderImportModal(files) {
    const header     = document.createElement('div');
    header.className = 'modal_header';

    const headerLabel       = document.createElement('span');
    headerLabel.className   = 'modal_header_label';
    headerLabel.textContent = 'Fichiers importés';
    header.appendChild(headerLabel);

    const headerActions     = document.createElement('div');
    headerActions.className = 'modal_header_actions';

    const closeBtn     = document.createElement('div');
    closeBtn.className = 'close-btn';
    closeBtn.title     = 'Fermer';
    closeBtn.innerHTML = '&#x2716;';

    closeBtn.addEventListener('click', e => {
      e.stopPropagation();
      document.getElementById('folder_modal').style.display = 'none';
    });

    headerActions.appendChild(closeBtn);
    header.appendChild(headerActions);
    
    const grid                 = document.createElement('div');
    grid.style.display         = 'flex';
    grid.style.flexWrap        = 'wrap';
    grid.style.gap             = '10px';
    grid.style.justifyContent  = 'center';

    for (const file of files) {
      
      const category = this.getFileCategory(file);

      const iconWrapper                 = document.createElement('div');
      iconWrapper.style.display         = 'flex';
      iconWrapper.style.flexDirection   = 'column';
      iconWrapper.style.alignItems      = 'center';
      iconWrapper.style.cursor          = 'pointer';


      const iconImg       = document.createElement('img');
      iconImg.src         = '/images/icons/file_icon.png';
      iconImg.alt         = file.name;
      iconImg.style.width = '64px';


      const iconLabel       = document.createElement('span');
      iconLabel.textContent = file.name;

      iconWrapper.appendChild(iconImg);
      iconWrapper.appendChild(iconLabel);

      iconWrapper.addEventListener('click', e => {
        e.stopPropagation();
        if (category) {
          this.readFile(file, category);
        } else {
          alert('Type de fichier non supporté.');
        }
      });

      grid.appendChild(iconWrapper);
    }
    this.showModal('folder_modal', header, grid, 'block');
  }

  /**
   * Lit récursivement un dossier et retourne ses fichiers et sous-dossiers.
   * @param {FileSystemEntry} directoryEntry - L'entrée du dossier.
   * @returns {Promise<Object>} Une promesse résolvant { files, directories }.
   */
  readDirectoryRecursive(directoryEntry) {
    return new Promise((resolve, reject) => {
      const files = [];
      const directories = [];
      let pending = 0;
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

  /**
   * Lit un dossier et affiche son contenu dans une modal.
   * @param {FileSystemEntry} directoryEntry - L'entrée du dossier.
   * @param {string} [modalId] - Identifiant optionnel pour la modal.
   */
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

  /**
   * Affiche le contenu d'un dossier dans une modal.
   * @param {string} modalId - L'identifiant de la modal.
   * @param {string} dirName - Le nom du dossier.
   * @param {Array<File>} files - Liste des fichiers.
   * @param {Array<Object>} directories - Liste des sous-dossiers.
   */
  showDirectoryModal(modalId, dirName, files, directories) {
    const header = document.createElement('div');
    header.className = 'modal_header';
    const headerLabel = document.createElement('span');
    headerLabel.className = 'modal_header_label';
    headerLabel.textContent = `Contenu du dossier : ${dirName}`;
    header.appendChild(headerLabel);
    const headerActions = document.createElement('div');
    headerActions.className = 'modal_header_actions';
    const closeBtn = document.createElement('div');
    closeBtn.className = 'close-btn';
    closeBtn.title = 'Fermer';
    closeBtn.innerHTML = '&#x2716;';
    closeBtn.addEventListener('click', e => {
      e.stopPropagation();
      document.getElementById(modalId).style.display = 'none';
    });
    headerActions.appendChild(closeBtn);
    header.appendChild(headerActions);

    const contentDiv = document.createElement('div');
    contentDiv.className = 'modal_content';
    contentDiv.style.width = '60vw';
    contentDiv.style.height = '60vh';
    contentDiv.style.overflow = 'auto';

    if (directories.length > 0) {
      const dirTitle = document.createElement('h3');
      dirTitle.textContent = 'Dossiers';
      contentDiv.appendChild(dirTitle);
      const dirGrid = document.createElement('div');
      dirGrid.style.display = 'flex';
      dirGrid.style.flexWrap = 'wrap';
      dirGrid.style.gap = '10px';
      dirGrid.style.justifyContent = 'center';
      directories.forEach(dir => {
        const dirWrapper = document.createElement('div');
        dirWrapper.style.display = 'flex';
        dirWrapper.style.flexDirection = 'column';
        dirWrapper.style.alignItems = 'center';
        dirWrapper.style.cursor = 'pointer';
        const folderIcon = document.createElement('img');
        folderIcon.src = '/images/icons/folder_icon.png';
        folderIcon.alt = dir.name;
        folderIcon.style.width = '64px';
        folderIcon.style.height = '64px';
        const folderLabel = document.createElement('span');
        folderLabel.textContent = dir.name;
        folderLabel.style.fontSize = '0.8em';
        folderLabel.style.textAlign = 'center';
        dirWrapper.appendChild(folderIcon);
        dirWrapper.appendChild(folderLabel);
        dirWrapper.addEventListener('click', e => {
          e.stopPropagation();
          this.readDirectory(dir.entry, 'directory_modal_' + Date.now());
        });
        dirGrid.appendChild(dirWrapper);
      });
      contentDiv.appendChild(dirGrid);
    }

    if (files.length > 0) {
      const fileTitle = document.createElement('h3');
      fileTitle.textContent = 'Fichiers';
      contentDiv.appendChild(fileTitle);
      const fileGrid = document.createElement('div');
      fileGrid.style.display = 'flex';
      fileGrid.style.flexWrap = 'wrap';
      fileGrid.style.gap = '10px';
      fileGrid.style.justifyContent = 'center';
      files.forEach(file => {
        const category = this.getFileCategory(file);
        const fileWrapper = document.createElement('div');
        fileWrapper.style.display = 'flex';
        fileWrapper.style.flexDirection = 'column';
        fileWrapper.style.alignItems = 'center';
        fileWrapper.style.cursor = 'pointer';
        const fileIcon = document.createElement('img');
        fileIcon.src = '/images/icons/file_icon.png';
        fileIcon.alt = file.name;
        fileIcon.style.width = '64px';
        fileIcon.style.height = '64px';
        const fileLabel = document.createElement('span');
        fileLabel.textContent = file.name;
        fileLabel.style.fontSize = '0.8em';
        fileLabel.style.textAlign = 'center';
        fileWrapper.appendChild(fileIcon);
        fileWrapper.appendChild(fileLabel);
        fileWrapper.addEventListener('click', e => {
          e.stopPropagation();
          if (category) {
            this.readFile(file, category);
          } else {
            alert('Type de fichier non supporté.');
          }
        });
        fileGrid.appendChild(fileWrapper);
      });
      contentDiv.appendChild(fileGrid);
    }
    this.showModal(modalId, header, contentDiv, 'flex');
  }
}
