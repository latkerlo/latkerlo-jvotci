With no options specified, the results for which strings are words, and what
types of words they are, should match the results provided by 
[jbovlaste](https://jbovlaste.lojban.org) (when you submit a new word) and 
[Camxes: Standard](http://lojban.github.io/ilmentufa/camxes.html). The 
following is a list of types of cases where the results produced by 
*latkerlo jvotci* are intentionally different from one or both of them. 

Any other discrepancies are probably bugs. Please open an issue or contact 
latkerlo if you find one!

<table>
  <tr>
    <th></th>
    <th>default settings</th>
    <th>JVS</th>
    <th>camxes: standard</th>
    <th></th>
  </tr>
  <tr>
    <td>kerlyspa'ykerlo</td>
    <td>✅ Brivla</td>
    <td>❌ Not Brivla</td>
    <td>❌ Not Brivla</td>
    <td rowspan="2">CVV, CVhV, and CCV forms are allowed on their own between 
y-hyphens.</td>
  </tr>
  <tr>
    <td>kerlyspaspa'ykerlo</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
  </tr>
  <tr>
    <td>spaspa'ykerlo</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
  </tr>
  <tr>
    <td>toispa'ykerlo</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
    <td>This is fine, actually: \*{spa'ykerlo} is illegal.</td>
  </tr>
  <tr><td colspan="5"></td></tr>
  <tr>
    <td>briiymle</td>
    <td>✅ Brivla</td>
    <td>❌ Not Brivla</td>
    <td>✅ Brivla</td>
    <td rowspan="3">zi'evla short rafsi can end with an on-glide.</td>
  </tr>
  <tr>
    <td>plukauaiymle</td>
    <td>✅ Brivla</td>
    <td>❌ Not Brivla</td>
    <td>✅ Brivla</td>
  </tr>
  <tr>
    <td>plukauai'ymle</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
  </tr>
  <tr><td colspan="5"></td></tr>
  <tr>
    <td>rai'y'ismu</td>
    <td>✅ Brivla</td>
    <td>❌ Not Brivla</td>
    <td>❌ Not Brivla</td>
    <td rowspan="2">In what world is "torai'y'ismu" a word?</td>
  </tr>
  <tr>
    <td>torai'y'ismu</td>
    <td>❌ Not Brivla</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
  </tr>
  <tr><td colspan="5"></td></tr>
  <tr>
    <td>toiysmu</td>
    <td>❌ Not Brivla</td>
    <td>❌ Not Brivla</td>
    <td>✅ Brivla</td>
    <td>Egregious.</td>
  </tr>
</table>
