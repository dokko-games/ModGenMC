<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import type { Project } from "../../../static/types"
    import RecentProject from "../components/RecentProject.svelte";

    let recent: Project[] = [];
    let error = "";
    let loading = true;

    async function loadRecent() {
        try {
            recent = await invoke<Project[]>("get_recent_projects");
        } catch (e) {
            console.error(e);
            error = `Failed to load recent projects: ${e}`;
        } finally {
            loading = false;
        }
    }

    onMount(loadRecent);
</script>

<div class="recent">
  <h1>Recent Projects</h1>

  {#if loading}
    <p class="status">Loading…</p>
  {:else if error}
    <p class="error">{error}</p>
  {:else if recent.length === 0}
    <p class="status">No recent projects</p>
  {:else}
    <ul class="recent-list">
      {#each recent as project}
        <RecentProject {project} />
      {/each}
    </ul>
  {/if}
</div>


<style>
    h1 {
  margin: 30px;
  margin-bottom: 10px;
}

.status {
  margin: 0 30px;
  margin-top: 8px;
  opacity: .8;
}
.error {
  margin: 0 30px;
  margin-top: 8px;
}

.recent-list {
  list-style: none;
  padding: 0;
  margin: 8px 30px;
}

.recent {
  display: flex;
  width: 100%;
  flex-direction: column;
}

</style>
