"use strict";(self.webpackChunka_mir_formality=self.webpackChunka_mir_formality||[]).push([[581],{3905:function(e,t,r){r.d(t,{Zo:function(){return c},kt:function(){return d}});var a=r(7294);function n(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function o(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,a)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?o(Object(r),!0).forEach((function(t){n(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):o(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function l(e,t){if(null==e)return{};var r,a,n=function(e,t){if(null==e)return{};var r,a,n={},o=Object.keys(e);for(a=0;a<o.length;a++)r=o[a],t.indexOf(r)>=0||(n[r]=e[r]);return n}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)r=o[a],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(n[r]=e[r])}return n}var s=a.createContext({}),u=function(e){var t=a.useContext(s),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},c=function(e){var t=u(e.components);return a.createElement(s.Provider,{value:t},e.children)},m={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},p=a.forwardRef((function(e,t){var r=e.components,n=e.mdxType,o=e.originalType,s=e.parentName,c=l(e,["components","mdxType","originalType","parentName"]),p=u(r),d=n,k=p["".concat(s,".").concat(d)]||p[d]||m[d]||o;return r?a.createElement(k,i(i({ref:t},c),{},{components:r})):a.createElement(k,i({ref:t},c))}));function d(e,t){var r=arguments,n=t&&t.mdxType;if("string"==typeof e||n){var o=r.length,i=new Array(o);i[0]=p;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l.mdxType="string"==typeof e?e:n,i[1]=l;for(var u=2;u<o;u++)i[u]=r[u];return a.createElement.apply(null,i)}return a.createElement.apply(null,r)}p.displayName="MDXCreateElement"},1959:function(e,t,r){r.r(t),r.d(t,{assets:function(){return c},contentTitle:function(){return s},default:function(){return d},frontMatter:function(){return l},metadata:function(){return u},toc:function(){return m}});var a=r(7462),n=r(3366),o=(r(7294),r(3905)),i=["components"],l={sidebar_position:2},s="Setup",u={unversionedId:"setup",id:"setup",title:"Setup",description:"How to build, test, and run a-mir-formality:",source:"@site/docs/setup.md",sourceDirName:".",slug:"/setup",permalink:"/a-mir-formality/docs/setup",draft:!1,editUrl:"https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/docs/setup.md",tags:[],version:"current",sidebarPosition:2,frontMatter:{sidebar_position:2},sidebar:"tutorialSidebar",previous:{title:"Introduction",permalink:"/a-mir-formality/docs/intro"},next:{title:"How formality works",permalink:"/a-mir-formality/docs/category/how-formality-works"}},c={},m=[{value:"Run racket manually for better stacktraces",id:"run-racket-manually-for-better-stacktraces",level:2},{value:"The <code>traced</code> macro",id:"the-traced-macro",level:2}],p={toc:m};function d(e){var t=e.components,r=(0,n.Z)(e,i);return(0,o.kt)("wrapper",(0,a.Z)({},p,r,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"setup"},"Setup"),(0,o.kt)("p",null,"How to build, test, and run a-mir-formality:"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("a",{parentName:"li",href:"https://download.racket-lang.org/"},"Download and install racket"),(0,o.kt)("ul",{parentName:"li"},(0,o.kt)("li",{parentName:"ul"},"run ",(0,o.kt)("inlineCode",{parentName:"li"},"nix-shell")," if you have nix available to get everything set up automatically"))),(0,o.kt)("li",{parentName:"ul"},"Check out the a-mir-formality repository"),(0,o.kt)("li",{parentName:"ul"},"Run ",(0,o.kt)("inlineCode",{parentName:"li"},"raco test -j22 src")," to run the tests",(0,o.kt)("ul",{parentName:"li"},(0,o.kt)("li",{parentName:"ul"},"This will use 22 parallel threads; you may want to tune the number depending on how many cores you have."))),(0,o.kt)("li",{parentName:"ul"},"You can use ",(0,o.kt)("a",{parentName:"li",href:"https://docs.racket-lang.org/drracket/"},"DrRacket"),", or you can use VSCode. We recommend the following VSCode extensions:",(0,o.kt)("ul",{parentName:"li"},(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("a",{parentName:"li",href:"https://marketplace.visualstudio.com/items?itemName=evzen-wybitul.magic-racket"},"Magic Racket")," to give a Racket mode that supports most LSP operations decently well"),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("a",{parentName:"li",href:"https://marketplace.visualstudio.com/items?itemName=2gua.rainbow-brackets"},"Rainbow brackets")," is highly recommended to help sort out ",(0,o.kt)("inlineCode",{parentName:"li"},"()")),(0,o.kt)("li",{parentName:"ul"},"The ",(0,o.kt)("a",{parentName:"li",href:"https://marketplace.visualstudio.com/items?itemName=studykit.unicode-math"},"Unicode Math")," or ",(0,o.kt)("a",{parentName:"li",href:"https://marketplace.visualstudio.com/items?itemName=oijaz.unicode-latex"},"Unicode Latex")," extensions are useful for inserting characters like ",(0,o.kt)("inlineCode",{parentName:"li"},"\u2200"),".")))),(0,o.kt)("h1",{id:"debugging-tips"},"Debugging tips"),(0,o.kt)("h2",{id:"run-racket-manually-for-better-stacktraces"},"Run racket manually for better stacktraces"),(0,o.kt)("p",null,"When you use ",(0,o.kt)("inlineCode",{parentName:"p"},"raco test"),", you often get stacktraces that are not very helpful. You can do better by running racket manually with the ",(0,o.kt)("inlineCode",{parentName:"p"},"-l errortrace")," flag, which adds some runtime overhead but tracks more information. The easiest way to do this is to use ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/nikomatsakis/a-mir-formality/blob/main/test"},"the ",(0,o.kt)("inlineCode",{parentName:"a"},"test")," script")," and give it the name of some ",(0,o.kt)("inlineCode",{parentName:"p"},"rkt")," file, e.g. ",(0,o.kt)("inlineCode",{parentName:"p"},"src/decl/test/copy.rkt"),". This will run the tests found in the ",(0,o.kt)("inlineCode",{parentName:"p"},"test")," submodule within that file."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-bash"},"./test src/decl/test/copy.rkt\n")),(0,o.kt)("p",null,"This will expand to running ",(0,o.kt)("inlineCode",{parentName:"p"},"racket")," with a command like this:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-bash"},"racket -l errortrace -l racket/base -e '(require (submod \"src/decl/test/copy.rkt\" test))' \n")),(0,o.kt)("h2",{id:"the-traced-macro"},"The ",(0,o.kt)("inlineCode",{parentName:"h2"},"traced")," macro"),(0,o.kt)("p",null,"The ",(0,o.kt)("inlineCode",{parentName:"p"},"(traced '() expr)")," macro is used to wrap tests throughout the code. The ",(0,o.kt)("inlineCode",{parentName:"p"},"'()")," is a list of metafunctions and judgments you want to trace. Just add the name of something in there, like ",(0,o.kt)("inlineCode",{parentName:"p"},"lang-item-ok-goals"),", and racket will print out the arguments when it is called, along with its return value:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-scheme"},"(traced '(lang-item-ok-goals) expr)\n")))}d.isMDXComponent=!0}}]);