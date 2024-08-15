// web/jsx/dict/action.jsx
var DictAction = {
  addDict: () => {
    console.log("addDict");
  },
  editDict: () => {
    console.log("editDict");
  },
  delDict: () => {
    console.log("delDict");
  }
};

// web/jsx/dict/index.jsx
window.onload = () => {
  const root = ReactDOM.createRoot(document.getElementById("app"));
  root.render(/* @__PURE__ */ React.createElement(DictView, null));
};
var DictView = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-app-fluid"
  }, /* @__PURE__ */ React.createElement(arco.Grid.Row, {
    guides: [1, 1],
    style: { height: "100vh" }
  }, /* @__PURE__ */ React.createElement(arco.Grid.Col, {
    flex: "260px"
  }, /* @__PURE__ */ React.createElement(DictTreeView, null)), /* @__PURE__ */ React.createElement(arco.Divider, {
    type: "vertical",
    style: { height: "100%" }
  }), /* @__PURE__ */ React.createElement(arco.Grid.Col, {
    flex: "auto",
    style: { height: "100%" }
  }, /* @__PURE__ */ React.createElement(DictGridView, null))));
};
var DictTreeView = () => {
  const treeData = [
    {
      title: "\u4E00\u7EA7 1",
      key: "0-0",
      children: [
        {
          title: "\u4E8C\u7EA7 1-0",
          key: "0-0-0",
          children: [
            {
              title: "\u4E09\u7EA7 1-0-0",
              key: "0-0-0-0"
            },
            {
              title: "\u4E09\u7EA7 1-0-1"
            }
          ]
        }
      ]
    }
  ];
  const DictTreeNodeMoreAction = () => {
    return /* @__PURE__ */ React.createElement(arco.Menu, {
      style: { width: "100px", background: "#fff", border: "1px solid #e8e8e8" }
    }, /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "1",
      onClick: () => {
        DictAction.addDict();
      }
    }, "\u65B0\u589E\u540C\u7EA7"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "2",
      onClick: () => {
        DictAction.addDict();
      }
    }, "\u65B0\u589E\u5B50\u7EA7"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "3",
      onClick: () => {
        DictAction.addDict();
      }
    }, "\u7F16\u8F91"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "4",
      onClick: () => {
        DictAction.addDict();
      }
    }, "\u5220\u9664"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "5",
      onClick: () => {
        DictAction.addDict();
      }
    }, "\u542F\u7528"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "6",
      onClick: () => {
        DictAction.addDict();
      }
    }, "\u505C\u7528"), /* @__PURE__ */ React.createElement(arco.Menu.Item, {
      key: "7",
      onClick: () => {
        DictAction.addDict();
      }
    }, "\u53D8\u66F4\u7236\u7EA7"));
  };
  const renderTreeNodExtra = (node) => {
    return /* @__PURE__ */ React.createElement(arco.Dropdown, {
      droplist: /* @__PURE__ */ React.createElement(DictTreeNodeMoreAction, null),
      position: "bl"
    }, /* @__PURE__ */ React.createElement(arcoicon.IconMore, {
      style: {
        position: "absolute",
        right: 8,
        fontSize: 12,
        top: 10,
        color: "#3370ff"
      }
    }));
  };
  return /* @__PURE__ */ React.createElement("div", null, /* @__PURE__ */ React.createElement("div", {
    style: { display: "block" }
  }, /* @__PURE__ */ React.createElement(arco.Input, {
    placeholder: "\u8BF7\u8F93\u5165",
    addAfter: /* @__PURE__ */ React.createElement(arcoicon.IconSearch, null)
  })), /* @__PURE__ */ React.createElement(arco.Tree, {
    treeData,
    blockNode: true,
    renderExtra: renderTreeNodExtra
  }));
};
var DictGridView = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-container"
  }, /* @__PURE__ */ React.createElement(DictGridSearchForm, null), /* @__PURE__ */ React.createElement(DictGridToolBar, null), /* @__PURE__ */ React.createElement(DictGridTable, null), /* @__PURE__ */ React.createElement(DictGridPage, null));
};
var DictGridSearchForm = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-search"
  }, "Grid");
};
var DictGridToolBar = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-toolbar"
  }, /* @__PURE__ */ React.createElement(arco.Button, {
    type: "primary"
  }, "\u65B0\u589E"));
};
var DictGridTable = () => {
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-table"
  }, "Grid");
};
var DictGridPage = () => {
  const [size, setSize] = React.useState("default");
  return /* @__PURE__ */ React.createElement("div", {
    className: "bmbp-grid-page"
  }, /* @__PURE__ */ React.createElement(arco.Pagination, {
    size,
    total: 50,
    showTotal: true,
    showJumper: true,
    sizeCanChange: true
  }));
};
