<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let raw_blocks: [string, number, number, number][] = [];
    let blocks: { name: string; id: number; atlas_id: number }[] = [];

    onMount(async () => {
        raw_blocks = await invoke("get_blocks_with_preview");
        blocks = raw_blocks.map(([name, id, atlas_id]) => ({
            name,
            id,
            atlas_id,
        }));
    });

    let leftWidth = 200;
    let rightWidth = 200;

    const startResizingLeft = (e: MouseEvent) => {
        const onMouseMove = (moveEvent: MouseEvent) => {
            if (moveEvent.clientX > 100 && moveEvent.clientX < 500) {
                leftWidth = moveEvent.clientX;
            }
        };

        const onMouseUp = () => {
            window.removeEventListener("mousemove", onMouseMove);
            window.removeEventListener("mouseup", onMouseUp);
        };

        window.addEventListener("mousemove", onMouseMove);
        window.addEventListener("mouseup", onMouseUp);
    };

    const startResizingRight = (e: MouseEvent) => {
        const onMouseMove = (moveEvent: MouseEvent) => {
            const newWidth = window.innerWidth - moveEvent.clientX;
            if (newWidth > 100 && newWidth < 500) {
                rightWidth = newWidth;
            }
        };

        const onMouseUp = () => {
            window.removeEventListener("mousemove", onMouseMove);
            window.removeEventListener("mouseup", onMouseUp);
        };

        window.addEventListener("mousemove", onMouseMove);
        window.addEventListener("mouseup", onMouseUp);
    };
</script>

<div class="flex flex-row w-full h-full">
    <div
        class="h-full bg-base-200 select-none overflow-auto border border-base-content/40"
        style="width: {leftWidth}px;"
    >
        <slot name="left" />
    </div>

    <button
        class="w-4 -ml-2 cursor-col-resize h-full opacity-0 z-999"
        aria-label="left-resizer"
        on:mousedown={startResizingLeft}
    ></button>

    <div class="flex-1 mt-8 bg-base-100 select-none">
        <div class="collapse collapse-arrow border border-base-content/20 mt-2">
            <input type="checkbox" />
            <div class="collapse-title">Blocks</div>
            <div
                class="grid gap-3 collapse-content"
                style="grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));"
            >
                {#each blocks as block}
                    <div
                        class="p-2 border border-base-content/40 bg-base-200 rounded-xl flex"
                    >
                        <div
                            class="absolute -m-2 w-10 h-10 border border-base-content/40 rounded-lg"
                        ></div>
                        <div
                            class="w-16 h-16 ml-1 mt-1 rounded-sm"
                            style="
                                        background-image: url('engine-asset://blocks/{block.id -
                                1}');
                                        background-size: auto; /* Keep original scale */
                                        background-position: -0px -0px;
                                        image-rendering: pixelated;
                                        width: 16px;
                                        height: 16px;
                                        transform: scale(2); /* Scale up so it's not tiny :3 */
                                        transform-origin: center;
                                    "
                        ></div>
                        <div class="ml-4.5">
                            {block.name.split(":").pop()}
                        </div>
                    </div>
                {/each}
            </div>
        </div>
    </div>

    <button
        class="w-4 -mr-2 cursor-col-resize h-full opacity-0 z-999"
        aria-label="right-resizer"
        on:mousedown={startResizingRight}
    ></button>

    <div
        class="h-full bg-base-200 select-none overflow-auto border border-base-content/40"
        style="width: {rightWidth}px;"
    >
        <slot name="right" />
    </div>
</div>
