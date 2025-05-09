<script module>
  import { SETTINGS } from "@/lib/stores";
</script>

<script>
  let { facts = [] } = $props();
</script>

<div class="facts above-map" class:hidden={facts.length === 0}>
  <ul>
    {#each facts as fact}
      <li
        class="mono spoiler"
        class:more-to-explore={fact.more_to_explore}
        class:hidden={fact.more_to_explore && $SETTINGS.hide_spoilers}>
        {fact.text}
      </li>
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
    max-height: 60%;
    width: 90%;

    border: 2px $color solid;
    background-color: var(--bg);
    font-size: 1.3em;
    overflow-y: auto;

    & ul {
      $m: 0.5em;

      list-style: none;
      margin: $m 0.5em $m -16px;
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
