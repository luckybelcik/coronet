<script lang="ts">
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
        class="h-full bg-base-200 select-none overflow-auto border border-base-300"
        style="width: {leftWidth}px;"
    >
        <slot name="left" />
    </div>

    <button
        class="w-4 -ml-2 cursor-col-resize h-full opacity-0 z-999"
        aria-label="left-resizer"
        on:mousedown={startResizingLeft}
    ></button>

    <div class="flex-1 h-full bg-base-100 select-none">
        <slot />
    </div>

    <button
        class="w-4 -mr-2 cursor-col-resize h-full opacity-0 z-999"
        aria-label="right-resizer"
        on:mousedown={startResizingRight}
    ></button>

    <div
        class="h-full bg-base-200 select-none overflow-auto border border-base-300"
        style="width: {rightWidth}px;"
    >
        <slot name="right" />
    </div>
</div>
