
<div class="content-row">
<div class="content-col">

{{#include ./template/README.md}}

</div>

<div class="content-col">

<div class="tab">
  <button class="maintab tablinks active" onclick="switchMainTab(event, 'Template')">Template</button>
  <button class="maintab tablinks" onclick="switchMainTab(event, 'Solution')">Solution</button>
  <button class="maintab tablinks" onclick="switchMainTab(event, 'Diff')">Diff</button>
</div>

<div id="Template" class="maintab tabcontent active">

No files edited in this step.

</div>

<div id="Solution" class="maintab tabcontent">

<div class="tab">
<button class="subtab tablinks file-solution file-modified active" onclick="switchSubTab(event, 'src/lib.rs')" data-id="src/lib.rs">src/lib.rs</button>
<button class="subtab tablinks file-solution file-modified" onclick="switchSubTab(event, 'src/liquidity_pool.rs')" data-id="src/liquidity_pool.rs">src/liquidity_pool.rs</button>
</div>
<div id="solution/src/lib.rs" class="subtab tabcontent active" data-id="src/lib.rs">

```rust
{{#include ./solution/src/lib.rs}}
```

</div>

<div id="solution/src/liquidity_pool.rs" class="subtab tabcontent" data-id="src/liquidity_pool.rs">

```rust
{{#include ./solution/src/liquidity_pool.rs}}
```

</div>



</div>

<div id="Diff" class="maintab tabcontent">


<div class="tab">
	<button class="difftab tablinks active" onclick="switchDiff(event, 'template.diff')" data-id="template.diff">template.diff</button>
	<button class="difftab tablinks" onclick="switchDiff(event, 'solution.diff')" data-id="solution.diff">solution.diff</button>
</div>
<div id="template.diff" class="difftab tabcontent active" data-id="template.diff">

```diff
{{#include ./template/template.diff}}
```

</div>
<div id="solution.diff" class="difftab tabcontent" data-id="solution.diff">

```diff
{{#include ./solution/solution.diff}}
```

</div>

</div>

</div>
</div>
