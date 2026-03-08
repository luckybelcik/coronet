<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";

    let projectNames: string[] = [];

    onMount(async () => {
        projectNames = await invoke("get_project_names");
    });

    async function handleLoadProject(selectedName: string) {
        if (!selectedName) return;

        try {
            await invoke("load_project", { projectName: selectedName });

            console.log(`${selectedName} loaded purrfectly :3`);

            await goto("/main_editor");
        } catch (error) {
            console.error("Failed to load project:", error);
        }
    }
</script>

<div class="w-full h-full flex items-center justify-center gap-5 select-none">
    <div class="dropdown">
        <div
            tabindex="0"
            role="button"
            class="btn btn-md border border-base-300"
        >
            Load Project
        </div>
        <ul
            tabindex="-1"
            class="dropdown-content menu rounded-box z-1 p-0 w-28.5 mt-2 shadow-sm border border-base-300 bg-base-200"
        >
            {#each projectNames as name}
                <li>
                    <button onclick={() => handleLoadProject(name)}>
                        {name}
                    </button>
                </li>
            {/each}
        </ul>
    </div>
    <button class="btn btn-md border border-base-300">Create Project</button>
</div>
