import "./index.css";

import React from "react";
import ReactDOM from "react-dom";

import sourceFile from "./source-file";

const rust = import("./pkg/hello_world");

function App() {
	return (
		<div>Hello, world!</div>
	);
}

function RobloxInstance(props) {
	const instance = props.tree.get_instance(props.id);

	const properties = instance.properties();

	let propertiesTitle = null;
	let propertiesDisplay = null;
	const propertiesList = [];

	for (const [key, value] of Object.entries(properties)) {
		propertiesList.push(
			<div key={ key } className="RobloxInstance-property">
				<span className="RobloxInstance-propertyName">
					{ key }:
				</span>
				<span className="RobloxInstance-propertyValue">
					{ String(value.Value) }
				</span>
			</div>
		);
	}

	if (propertiesList.length > 0) {
		propertiesTitle = <div className="RobloxInstance-propertiesTitle">Properties</div>;
		propertiesDisplay = <div className="RobloxInstance-properties">{ propertiesList }</div>;
	}

	const childrenIds = instance.get_children_ids();
	let childrenTitle = null;
	let childrenDisplay = null;

	if (childrenIds.length > 0) {
		childrenTitle = <div className="RobloxInstance-childrenTitle">Children</div>;

		const childrenList = [];

		for (const childId of childrenIds) {
			childrenList.push(<RobloxInstance key={ childId } tree={ props.tree } id={ childId } />);
		}

		childrenDisplay = <div className="RobloxInstance-children">{ childrenList }</div>;
	}

	return (
		<div className="RobloxInstance">
			<div className="RobloxInstance-name">{ instance.name() } ({ instance.class_name() })</div>
			{ propertiesTitle }
			{ propertiesDisplay }
			{ childrenTitle }
			{ childrenDisplay }
		</div>
	);
}

function RobloxTree(props) {
	const rootId = props.tree.get_root_id();

	return (
		<div>
			<RobloxInstance tree={ props.tree } id={ rootId } />
		</div>
	);
}

function main(m) {
	const root = document.createElement("div");
	root.id = "root";
	document.body.appendChild(root);

	const tree = m.parse_xml(sourceFile);
	const rootId = tree.get_root_id();

	ReactDOM.render(<RobloxTree tree={tree } />, root);
}

rust
	.then(main)
	.catch(console.error);