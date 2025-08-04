import React from 'react';
import Modal from 'react-bootstrap/Modal';

export default function Notice(props) {
    return <Modal
        show={!!props.notice}
        onHide={props.onClose}
        backdrop="static"
        keyboard={false}
        centered
    >
        <Modal.Header closeButton>
            <Modal.Title>{ props.title || "Info" }</Modal.Title>
        </Modal.Header>
        <Modal.Body>
            {props.notice}
        </Modal.Body>
    </Modal>
}