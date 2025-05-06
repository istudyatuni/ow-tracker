<script module>
  import { CATEGORIES } from "../lib/categories";
  import { t } from "../lib/i18n";
  import { SELECTED_CATEGORIES } from "../lib/stores";
</script>

<script>
  let changed = $state(false);
</script>

<div class="wrapper">
  <h4>{$t("shiplog-categories-header")}</h4>
  {#each Object.entries(CATEGORIES) as [i, id] (id)}
    <div>
      <label>
        <input
          type="checkbox"
          name={id}
          class={id}
          checked={$SELECTED_CATEGORIES[id]}
          onchange={(e) => {
            changed = true;
            SELECTED_CATEGORIES.set(id, e.target.checked);
          }} />
        {$t(`shiplog-category-${id}`)}
      </label>
    </div>
  {/each}
  <button onclick={() => window.location.reload()} disabled={!changed}
    >{$t("shiplog-categories-apply-button")}</button>
</div>

<style lang="scss">
  h4 {
    margin: 5px auto;
  }
  button {
    margin-top: 5px;
    margin-bottom: 10px;
  }
  .wrapper {
    background-color: var(--bg);
    border-radius: 10px;
    padding: 1px 10px;
    margin-bottom: 5px;
    max-width: 35em;
  }
</style>
