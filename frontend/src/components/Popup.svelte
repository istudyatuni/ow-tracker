<script>
  let { children } = $props();
</script>

<div class="outer">
  <div class="block">
    {#if children}
      {@render children()}
    {:else}
      fallback
    {/if}
  </div>
</div>

<style lang="scss">
  $color: #3280ff;

  .outer {
    display: flex;
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
  }
  .block {
    height: auto;
    width: auto;
    margin: auto;

    border: 2px $color solid;
    background-color: var(--bg);
    font-size: 1.3em;
    padding: 1em 3em;
  }
</style>
