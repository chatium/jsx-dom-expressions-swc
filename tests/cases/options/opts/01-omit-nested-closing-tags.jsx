// omitNestedClosingTags drops a closing tag whose parent will close it anyway.
const nested = <div><p>one</p><span>two</span><table><tbody><tr><td>x</td></tr></tbody></table></div>
const inline = <div><a>link</a><b>bold</b>tail</div>
