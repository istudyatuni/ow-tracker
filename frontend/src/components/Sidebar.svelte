<script>
  let opened = $state(false);

  function toggle_open() {
    opened = !opened;
  }
</script>

<div class="bar" class:border={opened}>
  <button onclick={toggle_open} class:hidden={opened}>Menu</button>
  <div class:hidden={!opened}>
    <button onclick={toggle_open}>Close</button>
    <br />
    <button>Upload save file</button>
  </div>
</div>

<style lang="scss">
  $color: #3280ff;

  .bar {
    position: absolute;
    top: 0;
    left: 0;

    height: auto;
    padding: 1em 1em;

    color: white;

    // above map
    z-index: 1000;
  }
  /*.border {
    border: 1px $color solid;
  }*/
  button {
    background: #082638;
    cursor: pointer;
    margin-bottom: 5px;
  }
  .hidden {
  	display: none;
  }
</style>
