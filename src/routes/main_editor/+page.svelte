<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { fileStore } from '../../lib/fileStore.svelte';
    import { goto } from '$app/navigation';

    let raw_blocks: [string, number, number, string][] = [];
    let blocks: { name: string; id: number; atlas_id: number, manifest_path: string }[] = $state([]);

    onMount(async () => {
        raw_blocks = await invoke("get_blocks_with_preview");
        blocks = raw_blocks.map(([name, id, atlas_id, manifest_path]) => ({
            name,
            id,
            atlas_id,
            manifest_path,
        }));
    });

    async function loadManifest(path: string) {
        try {
            const contents: string = await invoke("load_manifest", { path });
            fileStore.updateContentWithPath(contents, path);
            goto('/main_editor/manifest_editing');
        } catch (error) {
            console.error(error);
        }
    };

    let windowWidth = $state(0);

    $effect(() => {
        if (windowWidth) {
        }
    });
</script>

<!--we need do to this to force a layout change bc of the dumb collapse thingy every time the window changes size-->
<svelte:window bind:innerWidth={windowWidth} />

<div class="flex-1 mt-8 bg-base-100 select-none">
    <div class="collapse collapse-arrow border border-base-content/20 mt-2">
        <input type="checkbox" />
        <div class="collapse-title">Blocks</div>
        <div
            class="grid gap-3 collapse-content items-start"
            style="grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));"
        >
            {#each blocks as block}
                <button
                    class="relative border border-base-content/40 rounded-xl flex h-18"
                    onclick={() => loadManifest(block.manifest_path)}
                >
                    <div class="h-full w-18 rounded-xl z-9" style="background-image: url('engine-asset://blocks/{block.id - 1}'); background-size: 4.5rem; image-rendering: pixelated;">
                    </div>
                    <div class="p-2 z-2 flex-2">
                        <div class="flex flex-col w-full p-2 bg-base-100 rounded-md">
                            <div>
                                {block.name.split(":").pop()}
                            </div>
                            <div class="text-xs opacity-50">
                                {block.manifest_path.split("/").pop()}
                            </div>
                        </div>
                    </div>
                    <!-- svelte-ignore element_invalid_self_closing_tag -->
                    <div 
                        class="w-full h-full absolute left-0 bottom-0 opacity-30 rounded-xl"
                        style="background-image: url('engine-asset://blocks/{block.id - 1}'); background-size: 32px; image-rendering: pixelated;"
                    />
                </button>
            {/each}
        </div>
    </div>
</div>