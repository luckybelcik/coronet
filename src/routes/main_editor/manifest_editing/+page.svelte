<script lang="ts">
    import { fileStore } from '../../../lib/fileStore.svelte';
    import { invoke } from "@tauri-apps/api/core";
    import { getSingletonHighlighter, type Highlighter } from 'shiki';

    let highlighter: Highlighter | undefined = $state();
    let highlightedCode = $state("");

    getSingletonHighlighter({ themes: ['ayu-dark'], langs: ['toml'] }).then(h => highlighter = h);
    
    let timeout: number | undefined;

    $effect(() => {
        const code = fileStore.content;
        
        clearTimeout(timeout);
        timeout = setTimeout(async () => {
            if (highlighter) {
                highlightedCode = await highlighter.codeToHtml(code, {
                    lang: 'toml',
                    theme: 'ayu-dark',
                    transformers: [
                        {
                            pre(node) {
                                node.properties.style = ""; 
                            }
                        }
                    ]
                });
            }
        }, 1);
    });

    let error = $state(false);
    let button_color = $state("btn-success");

    async function saveManifest() {
        let path = fileStore.opened_path;
        let contents = fileStore.content;
        let went_ok = await invoke("update_manifest_file", { contents, path });
        if (went_ok) {
            error = false
            button_color = "btn-success";
        } else {
            error = true;
            button_color = "btn-error";
        }
    };

    let scrollContainer: HTMLElement | undefined  = $state();
    let textareaElement: HTMLElement | undefined  = $state();
    let highlightElement: HTMLElement | undefined = $state();

    function syncScroll() {
        if (textareaElement && highlightElement) {
            highlightElement.scrollTop = textareaElement.scrollTop;
            highlightElement.scrollLeft = textareaElement.scrollLeft;
        }
    }
</script>

<div class="relative w-full mt-8 font-mono text-sm bg-base-100 overflow-hidden">
    <div class="mt-2 px-4 py-2 text-xs flex items-center">
        <span>{fileStore.opened_path.split("/").pop()}</span>
    </div>

    <div class="relative h-full w-full">
        <div 
            bind:this={highlightElement}
            class="absolute inset-0 p-4 pointer-events-none overflow-hidden whitespace-pre"
        >
            {@html highlightedCode}
        </div>

        <textarea
            bind:this={textareaElement}
            bind:value={fileStore.content}
            onscroll={syncScroll}
            spellcheck="false"
            class="absolute inset-0 p-4 w-full h-full bg-transparent text-transparent caret-white resize-none outline-none whitespace-pre overflow-auto border-none"
        ></textarea>
    </div>

    <button class="absolute bottom-3 right-3 btn {button_color}" onclick={() => saveManifest()} aria-label="saveManifest">
        Save Manifest
    </button>
</div>

<style>
    div::-webkit-scrollbar { display: none; }
    
    textarea::-webkit-scrollbar {
        width: 10px;
    }
    textarea::-webkit-scrollbar-thumb {
        background: rgba(255, 255, 255, 0.1);
        border-radius: 5px;
    }
</style>