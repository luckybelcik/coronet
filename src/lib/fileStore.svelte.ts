export class FileStore {
    content = $state("");
    opened_path = $state("");

    constructor(initialContent = "", initialPath = "") {
        this.content = initialContent;
        this.opened_path = initialPath;
    }

    updateContent(newText: string) {
        this.content = newText;
    }

    updateContentWithPath(newText: string, newPath: string) {
        this.content = newText;
        this.opened_path = newPath;
    }
}

export const fileStore = new FileStore("Default file content...");