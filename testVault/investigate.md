---
title: Investigating
type: scene
---

# You Look Around

The room is sparse but well-kept. You notice a journal on the desk and a key under the bed.

[[journal|Read the journal]](player.curiosity += 15)
[[key|Take the key]](player.inventory = "key"; player.boldness += 10)
[[start|Go back]](player.curiosity -= 5)
