<script lang="ts">
  import type { Version } from "../../../static/types";

  export let version: Version;
</script>

{#if version.patch > 0}
  <div class="version">
    Fabric {version.major}.{version.minor}.{version.patch}
  </div>
  {:else}
  <div class="version">
    Fabric {version.major}.{version.minor}
  </div>
  {/if}

<style>
  
  .version {
    font-size: 0.85rem;
    opacity: 0.8;
    white-space: nowrap;
  }
</style>
