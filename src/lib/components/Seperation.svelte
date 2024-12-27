<div class="pattern">
	<div class="container">
	  <div class="pattern-inner"></div>
	</div>
</div>

<style>
	.pattern {
	  border-top: 2px solid var(--bg-overlay-selected);
	}

	.pattern-inner {
	  height: 40px;
	  /* Size settings remain the same */
	  background-size: 50px 3px, 
		50px 1px, 50px 1px, 50px 1px, 50px 1px,
		50px 1px, 50px 1px, 50px 1px, 50px 1px;
	  
	  /* Position settings remain the same */
	  background-position: 0 0, 
		22px 4px, 22px 5px, 22px 6px, 22px 7px,
		28px 12px, 28px 13px, 28px 14px, 28px 15px;
	  
	  /* The main change is in the background-image gradients */
	  background-image: 
		/* Main solid line at the top */
		linear-gradient(90deg, 
		  rgb(var(--element-color)), 
		  rgb(var(--element-color)) 10px, 
		  transparent 10px),
		
		/* First set of gradient lines */
		linear-gradient(90deg, 
		  rgba(var(--element-color), 0.4), rgba(var(--element-color), 0.4) 1px, 
		  rgba(var(--element-color), 0.8) 1px, rgba(var(--element-color), 0.8) 2px, 
		  rgba(var(--element-color), 0.9) 2px, rgba(var(--element-color), 0.9) 3px, 
		  rgba(var(--element-color), 0.4) 3px, rgba(var(--element-color), 0.4) 4px, 
		  transparent 4px, transparent 12px,
		  rgba(var(--element-color), 0.4) 12px, rgba(var(--element-color), 0.4) 13px,
		  rgba(var(--element-color), 0.8) 13px, rgba(var(--element-color), 0.8) 14px, 
		  rgba(var(--element-color), 0.9) 14px, rgba(var(--element-color), 0.9) 15px, 
		  rgba(var(--element-color), 0.4) 15px, rgba(var(--element-color), 0.4) 16px,
		  transparent 16px),
		linear-gradient(90deg, 
		  rgba(var(--element-color), 0.9), rgba(var(--element-color), 0.9) 1px, 
		  rgb(var(--element-color)) 1px, rgb(var(--element-color)) 2px, 
		  rgb(var(--element-color)) 2px, rgb(var(--element-color)) 3px, 
		  rgba(var(--element-color), 0.9) 3px, rgba(var(--element-color), 0.9) 4px, 
		  transparent 4px, transparent 12px,
		  rgba(var(--element-color), 0.9) 12px, rgba(var(--element-color), 0.9) 13px,
		  rgb(var(--element-color)) 13px, rgb(var(--element-color)) 14px, 
		  rgb(var(--element-color)) 14px, rgb(var(--element-color)) 15px, 
		  rgba(var(--element-color), 0.9) 15px, rgba(var(--element-color), 0.9) 16px,
		  transparent 16px),
		linear-gradient(90deg, 
		  rgba(var(--element-color), 0.9), rgba(var(--element-color), 0.9) 1px, 
		  rgb(var(--element-color)) 1px, rgb(var(--element-color)) 2px, 
		  rgb(var(--element-color)) 2px, rgb(var(--element-color)) 3px, 
		  rgba(var(--element-color), 0.9) 3px, rgba(var(--element-color), 0.9) 4px, 
		  transparent 4px, transparent 12px,
		  rgba(var(--element-color), 0.9) 12px, rgba(var(--element-color), 0.9) 13px,
		  rgb(var(--element-color)) 13px, rgb(var(--element-color)) 14px, 
		  rgb(var(--element-color)) 14px, rgb(var(--element-color)) 15px, 
		  rgba(var(--element-color), 0.9) 15px, rgba(var(--element-color), 0.9) 16px,
		  transparent 16px),
		linear-gradient(90deg, 
		  rgba(var(--element-color), 0.4), rgba(var(--element-color), 0.4) 1px, 
		  rgba(var(--element-color), 0.8) 1px, rgba(var(--element-color), 0.8) 2px, 
		  rgba(var(--element-color), 0.9) 2px, rgba(var(--element-color), 0.9) 3px, 
		  rgba(var(--element-color), 0.4) 3px, rgba(var(--element-color), 0.4) 4px, 
		  transparent 4px, transparent 12px,
		  rgba(var(--element-color), 0.4) 12px, rgba(var(--element-color), 0.4) 13px,
		  rgba(var(--element-color), 0.8) 13px, rgba(var(--element-color), 0.8) 14px, 
		  rgba(var(--element-color), 0.9) 14px, rgba(var(--element-color), 0.9) 15px, 
		  rgba(var(--element-color), 0.4) 15px, rgba(var(--element-color), 0.4) 16px,
		  transparent 16px),
		
		/* Second set of gradient lines */
		linear-gradient(90deg, 
		  rgba(var(--element-color), 0.4), rgba(var(--element-color), 0.4) 1px, 
		  rgba(var(--element-color), 0.8) 1px, rgba(var(--element-color), 0.8) 2px, 
		  rgba(var(--element-color), 0.9) 2px, rgba(var(--element-color), 0.9) 3px, 
		  rgba(var(--element-color), 0.4) 3px, rgba(var(--element-color), 0.4) 4px, 
		  transparent 4px),
		linear-gradient(90deg, 
		  rgba(var(--element-color), 0.9), rgba(var(--element-color), 0.9) 1px, 
		  rgb(var(--element-color)) 1px, rgb(var(--element-color)) 2px, 
		  rgb(var(--element-color)) 2px, rgb(var(--element-color)) 3px, 
		  rgba(var(--element-color), 0.9) 3px, rgba(var(--element-color), 0.9) 4px, 
		  transparent 4px),
		linear-gradient(90deg, 
		  rgba(var(--element-color), 0.9), rgba(var(--element-color), 0.9) 1px, 
		  rgb(var(--element-color)) 1px, rgb(var(--element-color)) 2px, 
		  rgb(var(--element-color)) 2px, rgb(var(--element-color)) 3px, 
		  rgba(var(--element-color), 0.9) 3px, rgba(var(--element-color), 0.9) 4px, 
		  transparent 4px),
		linear-gradient(90deg, 
		  rgba(var(--element-color), 0.4), rgba(var(--element-color), 0.4) 1px, 
		  rgba(var(--element-color), 0.8) 1px, rgba(var(--element-color), 0.8) 2px, 
		  rgba(var(--element-color), 0.9) 2px, rgba(var(--element-color), 0.9) 3px, 
		  rgba(var(--element-color), 0.4) 3px, rgba(var(--element-color), 0.4) 4px, 
		  transparent 4px);
	  background-repeat: repeat-x;
	}
</style>

