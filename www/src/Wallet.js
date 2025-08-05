import React, {useRef} from 'react';
import Modal from 'react-bootstrap/Modal';
import Form from 'react-bootstrap/Form';
import Button from 'react-bootstrap/Button';

export default function Notice(props) {
    const amountRef = useRef(null);
    const descriptorRef = useRef(null);

    if (!props.wallet) {
        return null;
    }

    let balance = new Intl.NumberFormat("de-De", {
        style: "currency",
        currency: props.wallet.currency,
    }).format(props.wallet.balance);

    const formSubmit = (evt) => {
        evt.preventDefault();
        if (!amountRef.current.value.match(/^\d+([.,]\d{2})?$/)) {
            return
        }
        let value = amountRef.current.value.replace(",", ".");
        props.onValue(value, descriptorRef.current.value || null);
    }

    return <Modal
        show={!!props.wallet}
        onHide={props.onClose}
        backdrop="static"
        keyboard={false}
        centered
    >
        <Modal.Header closeButton>
            <Modal.Title>Wallet { props.wallet.public_pan }</Modal.Title>
        </Modal.Header>
        <Modal.Body>
            { props.wallet.customer ? <p>Customer: { props.wallet.customer }</p> : null}
            <h2 className="mb-3"><small>Balance:</small><br/>{ balance }</h2>
            <Form onSubmit={formSubmit}>
                <Form.Group className="mb-3" controlId="formChargeDescriptor">
                    <Form.Label>Descriptor</Form.Label>
                    <Form.Control type="text" ref={descriptorRef} />
                </Form.Group>
                <Form.Group className="mb-3" controlId="formChargeAmount">
                    <Form.Label>Amount</Form.Label>
                    <Form.Control type="text" placeholder="0.00" inputMode="decimal" ref={amountRef} autoFocus required />
                </Form.Group>
                <Button variant="primary" type="submit">
                    Charge
                </Button>
            </Form>
        </Modal.Body>
    </Modal>
}