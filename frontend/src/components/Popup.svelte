<script>
  let { hidden, text } = $props()
</script>

<div class="block" class:hidden={hidden}>
  {text}
</div>

<style lang="scss">
  $color: #3280ff;

  .block {
    position: absolute;
    top: 45%;
    left: 5%;
    right: 5%;

    height: auto;

    border: 2px $color solid;
    background-color: var(--bg);
    font-size: 1.3em;
    padding: 1em;
    text-align: center;
  }
  .hidden {
    display: none;
  }
</style>
