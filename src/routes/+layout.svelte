<script lang="ts">
    import "../index.css";

    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { onMount } from "svelte";
    import { browser } from "$app/environment";

    const appWindow = getCurrentWindow();

    let { children } = $props();

    onMount(() => {
        if (appWindow != null) {
            const titlebar = document.getElementById("titlebar");

            if (titlebar != null) {
                document
                    .getElementById("titlebar")
                    ?.addEventListener("mousedown", (e) => {
                        if (e.buttons === 1) {
                            e.detail === 2
                                ? appWindow.toggleMaximize()
                                : appWindow.startDragging();
                        }
                    });
            }
        }
    });
</script>

<div class="w-full h-full absolute -z-999 bg-base-100"></div>

<div
    class="navbar bg-base-200 h-8 min-h-8 select-none border border-base-300"
    id="titlebar"
>
    <div class="flex-none px-2">
        <span class="text-xs opacity-50 sono font-medium">Coronet</span>
    </div>

    <div class="flex-1 text-center">
        <!-- svelte-ignore event_directive_deprecated -->
        <button
            class="btn btn-ghost btn-xs border hover:border-base-300"
            id="titlebar-file"
            on:mousedown|stopPropagation>File</button
        >
    </div>

    <!-- svelte-ignore event_directive_deprecated -->
    <div class="flex-none">
        <button
            on:mousedown|stopPropagation
            on:click={() => appWindow.minimize()}
            class="btn btn-ghost btn-xs border hover:border-base-300">_</button
        >
        <button
            on:mousedown|stopPropagation
            on:click={() => appWindow.toggleMaximize()}
            class="btn btn-ghost btn-xs border hover:border-base-300">□</button
        >
        <button
            on:mousedown|stopPropagation
            on:click={() => appWindow.close()}
            class="btn btn-ghost btn-xs hover:bg-error text-lg font-light border hover:border-base-300"
            >✕</button
        >
    </div>
</div>

{@render children()}
