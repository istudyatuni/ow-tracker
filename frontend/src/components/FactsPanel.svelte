<script>
  let { facts = [] } = $props();
</script>

<div class="facts" class:hidden={facts.length === 0}>
  <ul>
    {#each facts as fact}
      <li>{fact}</li>
    {/each}
  </ul>
</div>

<style lang="scss">
  $color: #3280ff;

  .facts {
    position: absolute;
    bottom: 2%;
    left: 5%;

    height: auto;
    min-height: 2em;
    width: 90%;

    border: 2px $color solid;
    background-color: #02101b;
    font-size: 1.3em;

    // above map
    z-index: 1000;

    & ul {
      $m: 0.5em;

      list-style: none;
      margin: $m auto $m -16px;
    }

    & ul li {
      font-family: monospace;
    }

    & ul li:before {
      content: "\2014 ";
      color: $color;
      margin-left: -1.5em;
      margin-right: 1em;
    }
  }
  .hidden {
    display: none;
  }
</style>
