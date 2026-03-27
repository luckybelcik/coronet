<script lang="ts">
    let { children } = $props();

    let leftWidth = $state(200);
    let rightWidth = $state(200);

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
    </div>

    <button
        class="w-4 -ml-2 cursor-col-resize h-full opacity-0 z-999"
        aria-label="left-resizer"
        onmousedown={startResizingLeft}
    ></button>

    {@render children()}

    <button
        class="w-4 -mr-2 cursor-col-resize h-full opacity-0 z-999"
        aria-label="right-resizer"
        onmousedown={startResizingRight}
    ></button>

    <div
        class="h-full bg-base-200 select-none overflow-auto border border-base-content/40"
        style="width: {rightWidth}px;"
    >
    </div>
</div>