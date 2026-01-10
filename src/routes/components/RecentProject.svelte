<script lang="ts">
  import type { Project } from "../../../static/types";
  import { invoke } from "@tauri-apps/api/core";

  export let project: Project;
  let error = "";
  async function handleOpen() {
      try {
        await invoke("open_project", {
          projectPath: project.path, // MUST match Rust parameter name
        });
      } catch (err) {
        error = `Error: ${err}`;
      }
    }
</script>

<li class="project" on:click={handleOpen}>
  <div class="name">{project.name}</div>
  <div class="path">{project.path}</div>
  {#if project.targetVersion.patch > 0}
  <div class="version">
    Fabric {project.targetVersion.major}.{project.targetVersion.minor}.{project.targetVersion.patch}
  </div>
  {:else}
  <div class="version">
    Fabric {project.targetVersion.major}.{project.targetVersion.minor}
  </div>
  {/if}
  {#if error}
  <p class="error">{error}</p>
  {/if}
</li>

<style>
  .project {
    padding: 8px 10px;
    border-radius: 6px;
    cursor: pointer;
    transition: 0.25s ease;
  }

  .project:hover {
    background: rgba(255, 255, 255, 0.05);
  }

  .name {
    font-weight: 600;
  }

  .version {
    font-size: 0.85rem;
    opacity: 0.8;
    white-space: nowrap;
  }

  .path {
    margin-top: 2px;
    opacity: 0.65;
    font-size: 0.85rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
