<script lang="ts">
    import { fileStore } from '../../../lib/fileStore.svelte';
    import { invoke } from "@tauri-apps/api/core";

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
</script>

<div class="relative w-full mt-8 font-mono text-sm border border-base-content/20 rounded-lg bg-base-100 overflow-hidden">
    <div class="grid p-4">
        <div 
            class="invisible whitespace-pre-wrap break-all row-start-1 col-start-1"
            aria-hidden="true"
        >
            {fileStore.content}
        </div>

        <textarea
            bind:value={fileStore.content}
            spellcheck="false"
            class="textarea textarea-ghost p-0 w-full row-start-1 col-start-1 resize-none focus:outline-none bg-transparent leading-normal"
        ></textarea>
    </div>

    <button class="absolute bottom-3 right-3 btn {button_color}" onclick={() => saveManifest()} aria-label="saveManifest">
        Save Manifest
    </button>
</div>