bun build --watch web/tsx/**/index.tsx --root=./web/tsx --outdir=./web/static/  --target=browser --asset-naming=css/[name].[ext] --entry-naming=js/[dir]/[name].[ext]   --external:react --external:react-dom --external:@arco-design/web-react --bundle
# // --minify
