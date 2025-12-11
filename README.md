# MetaViewer

MetaViewer is a modern metadata inspection and analysis tool written in Rust, designed as an improved and streamlined alternative to *exiftool*.  
It focuses on simplicity, speed, and extensibility, making it easier for security professionals, forensic analysts, developers, and analysts to extract and review metadata across multiple file formats.

## ✨ Features

- **Document-focused metadata extraction**: Quickly parse and display metadata from common office and text formats.  
- **Lightweight and efficient**: Built with performance in mind, avoiding unnecessary overhead.  
- **CLI-friendly design**: Ideal for scripting, automation, and integration into larger workflows.  
- **Extensible architecture**: Future-proof design to support additional file types and custom modules.  

## 📂 Supported File Types (Stage 1)

- **TXT** – Plain text files  
- **PDF** – Portable Document Format  
- **DOCX** – Microsoft Word documents  
- **XLSX** – Microsoft Excel spreadsheets  

> More file types will be implemented in later stages, expanding MetaViewer into a comprehensive metadata toolkit.

## 🚀 Roadmap

In the long run:
- Add support for images, audio, and video formats  
- Advanced filtering and export options (JSON, CSV)  
- Integration with security and OSINT workflows  
- Plugin system for community-driven extensions  

Next steps:
- 1.1: Implement help message (finished)
- 1.2: Enable support for .txt file analysis (finished)
- 1.3: Introduce analysis features for Word files (in progress)
- 1.4: Provide support for inspection of Excel files (not started)

## 🔧 Usage

MetaViewer is designed to be simple to use from the command line:

![MetaViewer Help Message](/img/help_msg.png)

```bash
metaviewer file.filextension
```

## 🗒️ Examples

Viewing info on a text file randomly found in my /home dir

![MetaViewer txt File Inspection](/img/txt_file_inspection.png)

